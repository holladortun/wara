use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueName {
    High,
    Default,
    Heavy,
}

impl QueueName {
    pub fn as_task_queue(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Default => "default",
            Self::Heavy => "heavy",
        }
    }
}

impl fmt::Display for QueueName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_task_queue())
    }
}

impl FromStr for QueueName {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "high" => Ok(Self::High),
            "default" => Ok(Self::Default),
            "heavy" => Ok(Self::Heavy),
            _ => Err("unsupported queue"),
        }
    }
}
