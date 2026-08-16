#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    namespace: String,
    path: String,
}

impl std::fmt::Display for Identifier {
    fn fmt(
        &self, f:
        &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}:{}",
            self.namespace,
            self.path
        ))
    }
}

impl Identifier {
    pub fn new(
        namespace: impl Into<String>,
        path: impl Into<String>
    ) -> Option<Self> {
        let namespace = namespace.into();
        let path = path.into();
        if namespace.is_empty() || path.is_empty() {
            return None;
        }
        Some(Self {
            namespace,
            path
        })
    }

    pub fn parse(
        raw: impl Into<String>
    ) -> Option<Self> {
        let raw = raw.into();
        let (namespace, path) = raw.split_once(':')?;
        Self::new(namespace, path)
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}