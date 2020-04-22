use crate::util::errors::Error::*;
use crate::Result;

pub fn domain(d: &str) -> Result<()> {
    if d.is_empty() {
        return Err(DomainError("domain cannot be empty".to_owned()));
    }

    Ok(())
}

pub fn path(p: &str) -> Result<()> {
    if p.is_empty() {
        return Err(PathError("path cannot be empty".to_owned()));
    }

    Ok(())
}

pub fn param_pair(k: &str, v: &str) -> Result<()> {
    if k.is_empty() {
        return Err(ParamError("key cannot be empty".to_owned()));
    }

    if v.is_empty() {
        return Err(ParamError("value cannot be empty".to_owned()));
    }

    Ok(())
}
