use std::collections::HashMap;
use std::collections::HashSet;

pub trait Plugin {
    fn execute(&self, input: &str) -> Result<String, String>;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
}

pub struct GreetPlugin;

impl Plugin for GreetPlugin {
    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(format!("Hello, {}!", input))
    }

    fn name(&self) -> &str {
        "GreetPlugin"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }
}

pub struct ReversePlugin;

impl Plugin for ReversePlugin {
    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(input.chars().rev().collect())
    }

    fn name(&self) -> &str {
        "ReversePlugin"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }
}

pub struct UppercasePlugin;

impl Plugin for UppercasePlugin {
    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(input.to_uppercase())
    }

    fn name(&self) -> &str {
        "UppercasePlugin"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }
}

pub struct PluginManager {
    registry: HashMap<String, Box<dyn Plugin>>,
    activable: HashSet<String>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            registry: HashMap::new(),
            activable: HashSet::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        let name = plugin.name().to_string();
        self.registry.insert(name.clone(), plugin);
        self.activable.insert(name);
    }
    pub fn unregister(&mut self, name: &str) -> Result<bool, String> {
        if self.registry.remove(name).is_some() {
            self.activable.remove(name);
            Ok(true)
        } else {
            Err("Plugin not found".to_string())
        }
    }

    pub fn get_plugin(&self, name: &str) -> Option<&Box<dyn Plugin>> {
        self.registry.get(name)
    }

    pub fn execute_plugin(&self, name: &str, input: &str) -> Result<String, String> {
        if !self.activable.contains(name) {
            return Err("Plugin is disabled".to_string());
        }
        if let Some(plugin) = self.registry.get(name) {
            plugin.execute(input)
        } else {
            Err("Plugin not found".to_string())
        }
    }

    pub fn enable(&mut self, name: &str) -> bool {
        if self.registry.contains_key(name) {
            self.activable.insert(name.to_string());
            true
        } else {
            false
        }
    }

    pub fn disable(&mut self, name: &str) -> bool {
        self.activable.remove(name)
    }

    
}