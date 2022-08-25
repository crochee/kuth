use regex::Regex;
use validator::ValidationError;

pub fn check_check(check: u8) -> Result<(), ValidationError> {
    if check == 1 || check == 2 {
        return Ok(());
    }
    Err(ValidationError::new("invalid check"))
}

lazy_static::lazy_static! {
    static ref SORT_REGEX: Regex = Regex::new(
        r"^[a-z][a-z_]{0,30}[a-z](\s(asc|ASC|desc|DESC))?(,[a-z][a-z_]{0,30}[a-z](\s(asc|ASC|desc|DESC))?)*$",
    ).unwrap();

    static ref PASSWORD_REGEX: Regex = Regex::new(
        r"^[a-zA-Z][a-zA-Z0-9_#@\$]{14,254}$",
    ).unwrap();
}

// 以字母开头，需要包含数字，字母，特殊字符（_,#,@,$）之一，长度不少于15位，最大不超过255位
pub fn check_password(password: &str) -> Result<(), ValidationError> {
    if PASSWORD_REGEX.is_match(password) {
        return Ok(());
    }
    Err(ValidationError::new("invalid password"))
}

pub fn check_sort(sort: &str) -> Result<(), ValidationError> {
    if SORT_REGEX.is_match(sort) {
        return Ok(());
    }
    Err(ValidationError::new("invalid sort"))
}

pub fn check_sex(sex: &str) -> Result<(), ValidationError> {
    if sex == "male" || sex == "female" {
        return Ok(());
    }
    Err(ValidationError::new("invalid sex"))
}

pub fn transform_sort(sort: &Option<String>) -> String {
    match sort {
        Some(v) => v.to_string(),
        None => "`created_at` desc".to_string(),
    }
}

const DEFAULT_LIMIT: u64 = 20;
const DEFAULT_OFFSET: u64 = 0;

pub fn transform_pagination(limit: Option<u64>, offset: Option<u64>) -> (u64, u64) {
    let limit_result = match limit {
        Some(mut temp_limit) => {
            if temp_limit == 0 {
                temp_limit = DEFAULT_LIMIT;
            }
            temp_limit
        }
        None => DEFAULT_LIMIT,
    };
    let offset_result = match offset {
        Some(mut temp_offset) => {
            if temp_offset == 0 {
                temp_offset = DEFAULT_OFFSET;
            }
            temp_offset
        }
        None => DEFAULT_OFFSET,
    };
    (limit_result, offset_result)
}

pub fn check_bind_type(bind_type: u8) -> Result<(), ValidationError> {
    if bind_type == 1 || bind_type == 2 {
        return Ok(());
    }
    Err(ValidationError::new("invalid bind_type"))
}

pub fn check_effect(effect: &str) -> Result<(), ValidationError> {
    if effect.eq("Allow") || effect.eq("Deny") {
        return Ok(());
    }
    Err(ValidationError::new("invalid effect"))
}

pub fn check_policy_type(policy_type: u8) -> Result<(), ValidationError> {
    if policy_type == 1 || policy_type == 2 {
        return Ok(());
    }
    Err(ValidationError::new("invalid policy_type"))
}
