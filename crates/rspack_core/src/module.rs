use std::fmt::Debug;

use crate::{Compilation, Dependency, ModuleDependency, ModuleGraph, ResolveKind, SourceType};

#[derive(Debug)]
pub struct ModuleGraphModule {
  // Only user defined entry module has name for now.
  pub name: Option<String>,
  pub id: String,
  pub exec_order: usize,
  pub uri: String,
  pub module: BoxModule,
  pub source_type: SourceType,
  all_dependecies: Vec<Dependency>,
}

impl ModuleGraphModule {
  pub fn new(
    name: Option<String>,
    id: String,
    uri: String,
    module: BoxModule,
    dependecies: Vec<Dependency>,
    source_type: SourceType,
  ) -> Self {
    Self {
      name,
      id,
      exec_order: usize::MAX,
      uri,
      module,
      all_dependecies: dependecies,
      source_type,
    }
  }

  pub fn depended_modules<'a>(&self, module_graph: &'a ModuleGraph) -> Vec<&'a ModuleGraphModule> {
    self
      .all_dependecies
      .iter()
      .filter(|dep| !matches!(dep.kind, ResolveKind::DynamicImport))
      .filter_map(|dep| module_graph.module_by_dependency(dep))
      .collect()
  }

  pub fn dynamic_depended_modules<'a>(
    &self,
    module_graph: &'a ModuleGraph,
  ) -> Vec<&'a ModuleGraphModule> {
    self
      .all_dependecies
      .iter()
      .filter(|dep| matches!(dep.kind, ResolveKind::DynamicImport))
      .filter_map(|dep| module_graph.module_by_dependency(dep))
      .collect()
  }
}

pub trait Module: Debug + Send + Sync {
  fn render(&self, module: &ModuleGraphModule, compilation: &Compilation) -> String;

  fn dependencies(&mut self) -> Vec<ModuleDependency> {
    vec![]
  }
}

pub type BoxModule = Box<dyn Module>;