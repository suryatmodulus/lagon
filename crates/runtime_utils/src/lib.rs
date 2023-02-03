use std::{
    collections::{HashMap, HashSet},
    env,
};

pub mod assets;
pub mod response;

#[derive(Debug, Clone)]
pub struct Deployment {
    pub id: String,
    pub function_id: String,
    pub function_name: String,
    pub domains: HashSet<String>,
    pub assets: HashSet<String>,
    pub environment_variables: HashMap<String, String>,
    pub memory: usize,          // in MB (MegaBytes)
    pub timeout: usize,         // in ms (MilliSeconds)
    pub startup_timeout: usize, // in ms (MilliSeconds)
    pub is_production: bool,
    pub cron: Option<String>,
}

impl Deployment {
    pub fn get_domains(&self) -> Vec<String> {
        let mut domains = Vec::new();

        domains.push(format!(
            "{}.{}",
            self.id,
            env::var("LAGON_ROOT_DOMAIN").expect("LAGON_ROOT_DOMAIN must be set")
        ));

        // Default domain (function's name) and custom domains are only set in production deployments
        if self.is_production {
            domains.push(format!(
                "{}.{}",
                self.function_name,
                env::var("LAGON_ROOT_DOMAIN").expect("LAGON_ROOT_DOMAIN must be set")
            ));

            domains.extend(self.domains.clone());
        }

        domains
    }

    pub fn should_run_cron(&self) -> bool {
        self.is_production && self.cron.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deployment_default_domains() {
        env::set_var("LAGON_ROOT_DOMAIN", "lagon.test");

        let deployment = Deployment {
            id: "123".into(),
            function_id: "456".into(),
            function_name: "hello".into(),
            domains: HashSet::new(),
            assets: HashSet::new(),
            environment_variables: HashMap::new(),
            memory: 128,
            timeout: 1000,
            startup_timeout: 1000,
            is_production: false,
            cron: None,
        };

        assert_eq!(deployment.get_domains(), vec!["123.lagon.test".to_owned()]);
    }

    #[test]
    fn deployment_domains() {
        env::set_var("LAGON_ROOT_DOMAIN", "lagon.test");

        let deployment = Deployment {
            id: "123".into(),
            function_id: "456".into(),
            function_name: "hello".into(),
            domains: HashSet::from_iter(vec!["lagon.app".to_owned()]),
            assets: HashSet::new(),
            environment_variables: HashMap::new(),
            memory: 128,
            timeout: 1000,
            startup_timeout: 1000,
            is_production: false,
            cron: None,
        };

        assert_eq!(deployment.get_domains(), vec!["123.lagon.test".to_owned(),]);
    }

    #[test]
    fn deployment_domains_production() {
        env::set_var("LAGON_ROOT_DOMAIN", "lagon.test");

        let deployment = Deployment {
            id: "123".into(),
            function_id: "456".into(),
            function_name: "hello".into(),
            domains: HashSet::from_iter(vec!["lagon.app".to_owned()]),
            assets: HashSet::new(),
            environment_variables: HashMap::new(),
            memory: 128,
            timeout: 1000,
            startup_timeout: 1000,
            is_production: true,
            cron: None,
        };

        assert_eq!(
            deployment.get_domains(),
            vec![
                "123.lagon.test".to_owned(),
                "hello.lagon.test".to_owned(),
                "lagon.app".to_owned()
            ]
        );
    }
}
