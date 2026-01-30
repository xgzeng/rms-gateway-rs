// functions to convert json properties names between lowerCamelCase and snake_case.
use serde_json::{Map, Value};

/// Convert all JSON object property names from lowerCamelCase to snake_case.
pub fn json_keys_to_snake_case(value: &Value) -> Value {
	match value {
		Value::Object(map) => {
			let mut converted = Map::with_capacity(map.len());
			for (key, val) in map.iter() {
				let new_key = lower_camel_to_snake(key);
				let new_val = json_keys_to_snake_case(val);
				converted.insert(new_key, new_val);
			}
			Value::Object(converted)
		}
		Value::Array(items) => {
			Value::Array(items.iter().map(json_keys_to_snake_case).collect())
		}
		_ => value.clone(),
	}
}

/// Convert all JSON object property names from snake_case to lowerCamelCase.
pub fn json_keys_to_lower_camel_case(value: &Value) -> Value {
	match value {
		Value::Object(map) => {
			let mut converted = Map::with_capacity(map.len());
			for (key, val) in map.iter() {
				let new_key = snake_to_lower_camel(key);
				let new_val = json_keys_to_lower_camel_case(val);
				converted.insert(new_key, new_val);
			}
			Value::Object(converted)
		}
		Value::Array(items) => {
			Value::Array(items.iter().map(json_keys_to_lower_camel_case).collect())
		}
		_ => value.clone(),
	}
}

fn lower_camel_to_snake(input: &str) -> String {
	if input.is_empty() {
		return String::new();
	}

	let mut result = String::with_capacity(input.len() + 4);
	let mut chars = input.chars().peekable();
	let mut prev_is_upper = false;
	let mut prev_is_lower_or_digit = false;

	while let Some(c) = chars.next() {
		let is_upper = c.is_uppercase();
		let is_lower = c.is_lowercase();
		let is_digit = c.is_ascii_digit();
		let next_is_lower = chars.peek().map_or(false, |n| n.is_lowercase());

		if is_upper {
			if !result.is_empty()
				&& (prev_is_lower_or_digit || (prev_is_upper && next_is_lower))
				&& !result.ends_with('_')
			{
				result.push('_');
			}
			result.extend(c.to_lowercase());
		} else {
			result.push(c);
		}

		prev_is_upper = is_upper;
		prev_is_lower_or_digit = is_lower || is_digit;
	}

	result
}

fn snake_to_lower_camel(input: &str) -> String {
	let mut parts = input.split('_').filter(|p| !p.is_empty());
	let mut result = String::with_capacity(input.len());

	if let Some(first) = parts.next() {
		result.push_str(&first.to_lowercase());
	}

	for part in parts {
		let mut chars = part.chars();
		if let Some(first_char) = chars.next() {
			result.extend(first_char.to_uppercase());
			result.push_str(&chars.as_str().to_lowercase());
		}
	}

	result
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;

	#[test]
	fn json_convert_camel_to_snake() {
		let value = json!({
			"beamConfig": { "beamId": 3, "httpURL": "x" },
			"gscList": [ { "gscId": 1 }, { "gscId": 2 } ]
		});

		let converted = json_keys_to_snake_case(&value);

		let expected = json!({
			"beam_config": { "beam_id": 3, "http_url": "x" },
			"gsc_list": [ { "gsc_id": 1 }, { "gsc_id": 2 } ]
		});

		assert_eq!(converted, expected);
	}

	#[test]
	fn json_convert_snake_to_camel() {
		let value = json!({
			"beam_config": { "beam_id": 3, "http_url": "x" },
			"gsc_list": [ { "gsc_id": 1 }, { "gsc_id": 2 } ]
		});

		let converted = json_keys_to_lower_camel_case(&value);

		let expected = json!({
			"beamConfig": { "beamId": 3, "httpUrl": "x" },
			"gscList": [ { "gscId": 1 }, { "gscId": 2 } ]
		});

		assert_eq!(converted, expected);
	}
}
