//! Error handling data: the hints of the API error codes (the errors table of the docs) and the
//! fields checked before a request is sent.

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

const JSON: &str = r#"{"docs":"https://docs.unione.io/en/web-api-ref#api-errors","hints":{"101":"Make sure you have passed correct X-API-KEY header or 'api_key' JSON parameter.","102":"Use correct user API-key (https://cp.unione.io/en/user/info/api) or project API key (https://cp.unione.io/en/user/project).","105":"You can turn API on here (https://cp.unione.io/en/user/info/api).","106":"To solve this problem, contact support (https://cp.unione.io/en/support).","111":"Check your JSON request syntax.","114":"Please send the request to the host you have been registered at, us1.unione.io or eu1.unione.io.","118":"Add a DNS verification record to your domain with values returned by domain/get-dns-records.","120":"Please contact support (https://cp.unione.io/en/support).","121":"DNS request has timed out. Please make sure your DNS provider is online and try again later.","122":"Please add to your domain a DKIM record with key returned by domain/get-dns-records method.","123":"You should properly setup DKIM for your domain.","124":"Seems like you have changed the record. While it's technically correct, it doesn't match the one we expect. You should use DKIM key returned by domain/get-dns-records method.","125":"Please setup the DKIM record for your domain properly.","126":"Please add a DKIM record for your domain with a key returned by domain/get-dns-records API method.","127":"A maximum of 10 DKIM records are allowed per primary domain.","129":"Please check that you have correctly added a DKIM record with key returned by domain/get-dns-records method and try again. Please contact support (https://cp.unione.io/en/support) if the error still occurs.","130":"Please use another domain.","150":"Unknown error occurred. Please try again later or contact support (https://cp.unione.io/en/support).","199":"The data in request should be no more than 10 MB.","201":"Check that your message body is being passed properly.","202":"Enter the sender email.","203":"Enter the subject.","204":"Make sure that the recipient's list is passed as an array where each recipient is an array with obligatory 'email' parameter.","205":"Make sure the parameter is passed properly.","206":"To avoid this error please use an email on a verified domain (https://docs.unione.io/en/web-api-ref#domain).","207":"Check the file and the file type again.","208":"Rename files with the forbidden symbols.","209":"Rename files with the same names.","210":"Rename files with the same names.","211":"Rename files with the forbidden symbols.","212":"The possible maximum substitutions number has been exceeded or substitutions have incorrect filetype (string/integer is allowed).","213":"Please enter a valid 'custom_backend_id' parameter.","215":"Try contacting support (https://cp.unione.io/en/support).","216":"'Metadata' value should be a string or integer. Array for 'metadata' key is expected.","217":"Maximum 10 metadata fields allowed.","218":"Metadata key length should be less than 64.","219":"Metadata value length should less than 1024.","220":"Make sure the metadata key is passed.","221":"Add the html <body> tag inside the <html> tag. The <body> tag is required to display HTML content.","223":"Add the 'content' and specify a value for it.","224":"Make sure the substitution name is passed correctly.","227":"Please enter a valid 'smtp_pool_id' parameter.","228":"Make sure the 'custom_backend_id' is passed.","229":"Make sure the custom backend domain or tracking domain is passed.","230":"Make sure the default backend or tracking domain is passed.","303":"Verify that the event list is entered correctly.","304":"Make sure the URL is passed correctly.","305":"Make sure the URL is passed correctly.","601":"Check the passed template id.","602":"Check the passed template name.","603":"Use another name for this template.","604":"Delete already created templates to create new ones. The number of templates is limited to 10000.","701":"Invalid reason for blocking email. Please contact support (https://cp.unione.io/en/support).","901":"Each user has a maximum daily limit for sending to prevent abuse. This limit increases automatically in case of good delivery and is reset each night. You can smoothly increase amount of daily sending or you can contact support and ask for manual limit increase.","902":"Wait till next day or switch to paid tariff.","903":"Please confirm ownership (https://docs.unione.io/en/web-api-ref#domain-validate-verification-record) of all domains you want to send emails to or switch tariff to the paid one.","904":"Please try another card or contact support (https://cp.unione.io/en/support) to resolve the situation.","905":"Usually tariff allows paid overage. In this case it's prohibited, possibly due to unpaid invoice. Please check that all invoices are paid or contact support (https://cp.unione.io/en/support) to solve the problem.","906":"You have used the entire available block reset limit. Try it tomorrow or contact support (https://cp.unione.io/en/support) to increase the limit.","908":"Please contact support (https://cp.unione.io/en/support).","1000":"Specify the recipient address.","1001":"Please enter a valid recipient address.","1002":"Specify the sender address.","1003":"Verify the sender domain address.","1004":"Please enter a valid sender address.","1005":"The sender name must be shorter. Please correct this value.","1006":"Try to repeat the call the next day.","1007":"Use an address from a confirmed domain.","1100":"Specify the domain to be verified.","1101":"Please enter a valid domain value.","1102":"Please use another domain.","1103":"Use only your own domains.","1104":"Please use another domain.","1200":"Specify the domain for validating.","1201":"Please enter a valid domain value.","1202":"Make a domain/get-dns-records method call.","1203":"Please use another domain.","1301":"Specify the domain to be verified.","1302":"The value of limit must be an integer in the range of 1 - 100.","1303":"The value of offset must be greater than or equal to 0.","1304":"The value of offset must be an integer.","1305":"The value of limit must be an integer.","1306":"Please use another domain.","1350":"Make sure the 'domain' is passed.","1351":"Please enter a valid domain value.","1352":"Make sure the 'domain' is passed.","1353":"Please use another domain.","1400":"Specify the domain for validating.","1401":"Either you have mistyped the domain name and should correct it or DNS changes have not yet propagated through DNS and you should wait a couple of hours and try again. There should be A, AAAA or MX record for domain.","1402":"Please call domain/get-dns-records method for domain to be registered.","1403":"Please use another domain.","1506":"Please check your JSON and use object for 'headers' field, not array or string or whatever.","1507":"Please limit 'headers' to no more than 50 elements.","1508":"Make sure the template ID is correct.","1510":"Use string type for 'body.html' parameter.","1511":"Use string type for 'body.plaintext' parameter.","1512":"Use string type for 'body.amp' parameter.","1516":"Check the correct substitutions in the 'body.html' field.","1517":"Check the correct substitutions in the 'body.plaintext' field.","1518":"Check the correct substitutions in the 'body.amp' field.","1519":"Use string type for 'reply_to' parameter.","1520":"Check if the email is correct.","1521":"Use string type for 'from_name' parameter.","1522":"Use string type for 'subject' parameter.","1523":"Please pass one of the following values: 'simple', 'liquid', 'velocity' or 'none'.","1524":"Please limit 'recipients' to no more than 500 elements.","1525":"Make sure the date and time of the scheduled sending is no more than 24 hours in advance.","1526":"Please pass only one of the following values: 'velocity', 'liquid', 'simple' or 'none'.","1527":"Please pass only one of 'be', 'de', 'en', 'es', 'fr', 'it', 'kz', 'ru', 'pl', 'pt', 'ua' values.","1528":"Please pass only one of 'be', 'de', 'en', 'es', 'fr', 'it', 'kz', 'ru', 'pl', 'pt', 'ua' values.","1529":"Make sure the datetime is passed correctly.","1530":"Use string type for 'options.send_at' parameter. Make sure the datetime is passed correctly.","1532":"Please limit tags to no more than 4 elements.","1533":"Delete already created tags to create new ones. The number of tags is limited to 10000.","1534":"The tags must be shorter. Please correct this value.","1535":"Make sure the tags is passed correctly.","1537":"Set parameter value to 1.","1538":"contact support (https://cp.unione.io/en/support) to set the flag.","1539":"contact support (https://cp.unione.io/en/support) to set the flag.","1540":"Metadata value size should less than 4000 bytes.","1541":"Metadata value size should less than 4000 bytes.","1542":"Metadata value size should less than 4000 bytes.","1543":"Use string type for 'global_metadata.campaign_id' parameter.","1544":"Use string type for 'metadata.campaign_id' parameter.","1545":"Make sure the 'metadata.campaign_id' is passed correctly.","1546":"Check the correct substitutions in the 'subject' field.","1547":"Check the correct substitutions in the 'from_name' field.","1548":"Use object type for the 'options.unsubscribe_url' parameter.","1549":"Make sure the substitution name is passed correctly.","1550":"Use object type for the 'options' parameter.","1551":"Use string type for 'metadata' and 'global_metadata' parameters.","1554":"Make sure that the number of emails in 'headers['Cc']' is not exceeded.","1555":"Use only emails from the 'recipient' parameter.","1556":"Make sure that the 'headers['To']' is passed.","1557":"Make sure that the number of emails in 'headers['To']' is not exceeded.","1558":"Use only emails from the 'recipient' parameter.","1559":"BCC may contain only addresses on your verified domain.","1560":"Make sure that the number of emails in \"To\" and \"Cc\" headers is not exceeded.","1561":"Make sure the headers is passed correctly.","1562":"Make sure the headers is passed correctly.","1565":"Make sure the substitution name is passed correctly.","1566":"Make sure the substitution name is passed correctly.","1567":"Make sure the substitution name is passed correctly.","1568":"Make sure the substitution name is passed correctly.","1569":"Make sure the substitution name is passed correctly.","1570":"Make sure the substitution name is passed correctly.","1571":"Make sure the substitution name is passed correctly.","1572":"The key must be shorter. Please correct this value.","1573":"Make sure the key is correct.","1574":"To avoid this error please use an email on a verified domain.","1575":"Try contacting support (https://cp.unione.io/en/support).","1576":"Enter the subject.","1577":"Check that your message body is being passed properly.","1578":"Add the html <body> tag inside the <html> tag. The <body> tag is required to display HTML content.","1579":"The possible maximum substitutions number has been exceeded.","1580":"Make sure the substitution name is passed correctly.","1581":"This value should be a string or integer.","1582":"Maximum 10 metadata fields allowed.","1583":"'Metadata' value should be a string or integer.","1584":"Make sure the metadata key is passed.","1585":"Make sure the metadata key is passed correctly.","1586":"It should have 1024 characters or less.","1587":"Try contacting support (https://cp.unione.io/en/support).","1588":"Try contacting support (https://cp.unione.io/en/support).","1590":"Rename files with the forbidden symbols.","1591":"Rename files with the same names.","1592":"Add the 'content' and specify a value for it.","1593":"Check the file and the file type again.","1594":"Rename files with the forbidden symbols.","1595":"Rename files with the same names.","1596":"Add the 'content' and specify a value for it.","1597":"Check the file and the file type again.","1598":"To avoid this error please use only confirmed email addresses.","1599":"Try to repeat the call the next day.","1601":"Correct JSON structure.","1602":"Use non-blank name.","1603":"Use string type for 'name' parameter.","1604":"Use shorter name.","1605":"Use unique project name or delete existing project first.","1606":"Set parameter value to true or false only.","1607":"Set parameter value to true or false only.","1608":"Please contact support (https://cp.unione.io/en/support).","1609":"Use string type for 'country' parameter.","1610":"Make sure the ISO-3166 alpha-2 country code is passed correctly.","1611":"Use int type for 'backend_domain_id' parameter.","1612":"Please enter a valid 'backend_domain_id' parameter.","1613":"Please enter a valid 'backend_domain_id' parameter.","1614":"Use int type for 'unsubscribe_page_id' parameter.","1701":"Correct JSON structure.","1702":"Use non-blank name.","1703":"Use string type for 'name' parameter.","1704":"Use shorter name.","1705":"Use unique project name or delete existing project first.","1706":"Don't forget to send either project_id or project_api_key.","1707":"Use correct project_api_key. You can get a list of projects with their keys with project/list (https://docs.unione.io/en/web-api-ref#project-list) method.","1708":"Use string type for 'project_api_key'.","1709":"Set parameter value to true or false only.","1710":"Set parameter value to true or false only.","1711":"Please contact support (https://cp.unione.io/en/support).","1712":"Please use string type for project_id field.","1713":"Please pass only one of the two parameters: either project_id or project_api_key, not both.","1714":"Project id is invalid. Maybe it was deleted or never existed?","1715":"Please check that you pass the project_id you have received by calling project/create (https://docs.unione.io/en/web-api-ref#project-create) method.","1716":"Use string type for 'country' parameter.","1717":"Make sure the ISO-3166 alpha-2 country code is passed correctly.","1718":"Use int type for 'backend_domain_id' parameter.","1719":"Please enter a valid 'backend_domain_id' parameter.","1720":"Please enter a valid 'backend_domain_id' parameter.","1721":"Use int type for 'email_counter' parameter.","1722":"Set parameter value equal to 0.","1723":"Use int type for 'email_counter_limit' parameter.","1724":"Set parameter value greater than or equal to 0.","1725":"Set parameter value to default or permanent only.","1726":"Use int type for 'unsubscribe_page_id' parameter.","1801":"Don't forget to send either project_id or project_api_key.","1802":"Use correct project_api_key. You can get a list of projects with their keys with project/list (https://docs.unione.io/en/web-api-ref#project-list) method.","1803":"Use string type for 'project_api_key'.","1804":"Please pass only one of the two parameters: either project_id or project_api_key, not both.","1805":"Please use string type for project_id field.","1806":"Project id is invalid. Maybe it was deleted or never existed?","1807":"Please check that you pass the project_id you have received by calling project/create (https://docs.unione.io/en/web-api-ref#project-create) method.","1902":"Use string type for 'project_api_key'.","1904":"Use correct numbers-only format for 'project_id'.","1905":"Please use string type for project_id field.","2306":"Make sure the 'recipients' is passed.","2307":"Use string type for 'recipients' parameter.","2500":"Make sure the 'editor_type' field is passed 'html'.","2501":"Make sure the template is created in HTML editor.","2502":"Pass 'html' in 'editor_type' field.","2503":"Make sure the template ID is correct.","2504":"Please check your JSON and use object for 'headers' field, not array or string or whatever.","2505":"Please limit headers to no more than 50 elements.","2514":"Use string type for 'reply_to' parameter.","2515":"Check if the email is correct.","2516":"Please pass one of the following values: 'simple', 'liquid', 'velocity' or 'none'.","2517":"Use string type for 'subject' parameter.","2518":"Check the correct substitutions in the 'subject' field.","2519":"Use string type for 'from_name' parameter.","2520":"Check the correct substitutions in the 'from_name' field.","2521":"Make sure the substitution name is passed correctly.","2522":"Make sure the substitution name is passed correctly.","2523":"Make sure the substitution name is passed correctly.","2524":"Use string type for 'options' parameter.","2525":"Make sure the substitution name is passed correctly.","2526":"Make sure the substitution name is passed correctly.","2527":"Use object type for the 'options.unsubscribe_url' parameter.","2528":"Use shorter name.","2529":"Use shorter name.","2530":"Make sure the substitution name is passed correctly.","2531":"Make sure the substitution name is passed correctly.","2532":"Make sure the substitution name is passed correctly.","2533":"Make sure the substitution name is passed correctly.","2534":"Make sure the substitution name is passed correctly.","2535":"Make sure the substitution name is passed correctly.","2536":"Make sure the substitution name is passed correctly.","2600":"Set parameter value greater than or equal to 0.","2601":"Use int type for 'limit' parameter.","2602":"Set parameter value greater than or equal to 0.","2603":"Use int type for 'offset' parameter.","2700":"Use string type for 'url' parameter.","2701":"Make sure the URL is passed.","2702":"Set parameter value greater than or equal to 0.","2703":"Use format 'json_post' or 'json_post_gzip' only.","2704":"Set parameter 'delivery_info' value to 1\\0 either true\\false.","2705":"Set parameter 'single_event' value to 1\\0 either true\\false.","2706":"Use int type for 'maxParallel' parameter.","2707":"Use a valid value of 5 to 100.","2708":"Use array type for 'events' parameter.","2709":"Use non-blank events.","2710":"Add 'email_status' parameter.","2711":"Add valid email statuses in the value.","2712":"Add valid spam blocking events in the value.","2713":"Make sure the domain is passed correctly.","2714":"Set status 'active' or 'disabled'.","2715":"Use string type for 'status' parameter.","2800":"Make sure the 'name' is passed.","2801":"Use string type for 'name' parameter.","2802":"Use string type for 'type' parameter.","2803":"Use string type for 'content' parameter.","2804":"Reduce the size of the 'content' field to 7MB (9786710 bytes in base64).","2900":"Make sure the email is passed.","2901":"Use string type for 'email' parameter.","2902":"Verify that email is entered correctly.","2903":"Set parameter value to true or false only.","3000":"Make sure the email is passed.","3001":"Use string type for 'email' parameter.","3002":"Verify that email is entered correctly.","3003":"Email was not found in the suppression list.","3004":"Email cannot be removed from the suppression list. Probably because of the severe reason like 'blocked'.","3100":"Make sure the email is passed.","3101":"Use string type for 'email' parameter.","3102":"Verify that email is entered correctly.","3103":"Make sure the 'cause' is passed.","3104":"Use string type for 'country' parameter.","3105":"Please pass only one of the available values.","3106":"Use string type for 'created' parameter.","3107":"Make sure the datetime is passed correctly.","3200":"Use string type for 'country' parameter.","3201":"Please pass only one of the available values.","3202":"Use string type for 'source' parameter.","3203":"Please pass only one of the available values.","3204":"Use string type for 'start_time' parameter.","3205":"Make sure the datetime is passed correctly.","3206":"Use string type for 'cursor' parameter.","3207":"Please pass only one of the available values.","3208":"Use int type for 'limit' parameter.","3209":"Set parameter value greater than or equal to 0.","3300":"Make sure the 'start_time' is passed.","3301":"Make sure the datetime is passed correctly.","3302":"Make sure the datetime is passed correctly.","3303":"Set parameter value greater than or equal to 0.","3304":"Use 'csv' or 'csv_gzip' format only.","3305":"Use 'delimiter' ';' or ',' only.","3308":"Projects methods are disabled by default. To enable them,contact support (https://cp.unione.io/en/support).","3309":"Delete already created dumps to create new ones. The number of dumps is limited to 10.","3310":"Make sure 'start_time' is in the allowed range.","3311":"Use int type for 'limit' parameter.","3312":"Use string type for 'delimiter' parameter.","3313":"Use string type for 'format' parameter.","3315":"Use string type for 'aggregate' parameter.","3316":"Can not use filter with \"aggregate\" parameter. Please pass only one of the available parameter.","3317":"Use array type for 'dumpFields' parameter.","3318":"Please pass only one of the available values.","3319":"Invalid value, aggregate could be only 'day_status' or empty. Please pass only one of the available values.","3400":"Please check your JSON and use object for 'filter' field, not array or string or whatever.","3401":"Use string type for 'filter.job_id' parameter.","3402":"Use string type for 'filter.status' parameter.","3403":"Please enter a valid 'filter.status' parameter.","3404":"Use string type for 'filter.email' parameter.","3405":"Check if the email is correct.","3406":"Use string type for 'filter.email_from' parameter.","3407":"Check if the email is correct.","3408":"Use string type for 'filter.domain' parameter.","3409":"Please enter a valid domain value.","3410":"Make sure the domain is passed correctly.","3411":"Use string type for 'filter.delivery_status' parameter.","3412":"Please enter a valid 'filter.delivery_status' parameter.","3413":"Use string type for 'filter.campaign_id' parameter.","3414":"The data in request should be no more than 128 bit.","3500":"Make sure the 'dump_id' is passed.","3501":"Make sure the 'dump_id' is passed correctly.","3502":"Dump ID is invalid. Maybe it was deleted or never existed?","3600":"Make sure the 'dump_id' is passed.","3601":"Make sure the 'dump_id' is passed correctly.","3602":"Dump ID is invalid. Maybe it was deleted or never existed?","3700":"Make sure the 'tag_id' is passed.","3701":"Use int type for 'tag_id' parameter.","3800":"Reduce the the numbers of concurrent requests to 4.","3801":"Make sure the email is passed.","3802":"Use string type for 'email' parameter.","3803":"Try to repeat the call the next day.","3900":"Use string type for 'idempotence_key' parameter.","4000":"Use the main account API key.","4001":"Pass an email address.","4002":"Use string type for 'email' parameter.","4003":"Pass a valid email address.","4004":"Delete it and call confirmed-email/create again to send a new confirmation link.","4005":"No action is required.","4006":"Delete an address before adding a new one.","4100":"Use the main account API key.","4101":"Pass an email address.","4102":"Use string type for 'email' parameter.","4103":"Pass a valid email address.","4104":"Use confirmed-email/list to get the current list.","4200":"Use the main account API key.","4300":"Use the account API key.","10001":"Use shorter name.","10002":"Use shorter name.","10003":"Make sure the substitution name is passed correctly.","10004":"Make sure the substitution name is passed correctly.","10005":"Make sure the substitution name is passed correctly.","10006":"Make sure the substitution name is passed correctly.","10007":"Make sure the substitution name is passed correctly.","10008":"Make sure the substitution name is passed correctly.","10009":"Make sure the substitution name is passed correctly."},"roots":{"email/send":"EmailSendRequest"},"models":{"EmailSendRequest":[["message",true,"MessageObject","one"]],"MessageObject":[["recipients",true,"MessageObjectRecipientsItem","list"]],"MessageObjectRecipientsItem":[["email",true,null,"one"]]}}"#;

/// A field checked before a request: wire name, required, nested model, "one" | "list" | "map".
type Field = (String, bool, Option<String>, String);

#[derive(Deserialize, Default)]
struct Data {
    #[serde(default)]
    docs: String,
    #[serde(default)]
    hints: HashMap<String, String>,
    #[serde(default)]
    roots: HashMap<String, String>,
    #[serde(default)]
    models: HashMap<String, Vec<Field>>,
}

const TRIM: &[char] = &['.', ' '];

fn data() -> &'static Data {
    static DATA: OnceLock<Data> = OnceLock::new();
    DATA.get_or_init(parse)
}

fn parse() -> Data {
    serde_json::from_str(JSON).unwrap_or_default()
}

/// API method of a path: "/email/send.json" -> "email/send".
pub(crate) fn method_of(path: &str) -> String {
    let path = path.trim_matches('/');
    path.strip_suffix(".json").unwrap_or(path).to_string()
}

/// How to fix an API error: the errors table of the docs, `None` for an unknown code.
pub(crate) fn hint(code: Option<i64>) -> Option<String> {
    data().hints.get(&code?.to_string()).cloned()
}

/// The first required field missing in the body of a method, e.g. "message.recipients[0].email".
pub(crate) fn missing(method: &str, body: &Value) -> Option<String> {
    walk(body, data().roots.get(method)?, "")
}

/// Message of an API error:
/// "email/send: Unknown recipient (API error 204, HTTP 400). Fix: ... See ...".
pub(crate) fn api_message(method: &str, text: &str, status: u16, code: Option<i64>) -> String {
    let text = quote(text);
    let text = text.trim_end_matches(TRIM);
    let mut out = if method.is_empty() {
        String::new()
    } else {
        format!("{method}: ")
    };
    out.push_str(if text.is_empty() { "API error" } else { text });
    let status_text = match code {
        Some(code) => format!(" (API error {code}, HTTP {status})."),
        None => format!(" (HTTP {status})."),
    };
    out.push_str(&status_text);
    if let Some(hint) = hint(code) {
        out.push_str(" Fix: ");
        out.push_str(hint.trim_end_matches(TRIM));
        out.push('.');
    }
    let docs = &data().docs;
    if !docs.is_empty() {
        out.push_str(" See ");
        out.push_str(docs);
    }
    out
}

/// A required field counts as missing when it is absent, null, "" or an empty list.
fn is_empty(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(text) => text.is_empty(),
        Value::Array(items) => items.is_empty(),
        _ => false,
    }
}

/// API error code: a JSON number or a string of digits only, `None` for anything else.
pub(crate) fn code_of(value: Option<&Value>) -> Option<i64> {
    match value? {
        Value::Number(number) => number.as_u64().and_then(|code| i64::try_from(code).ok()),
        Value::String(text) if is_digits(text) => text.parse().ok(),
        _ => None,
    }
}

fn is_digits(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit())
}

/// A server text (an API message, a response body) cut to 500 characters, for the messages.
pub(crate) fn quote(text: &str) -> String {
    const MAX_QUOTE: usize = 500;
    match text.char_indices().nth(MAX_QUOTE) {
        None => text.to_string(),
        Some((end, _)) => format!("{}... ({} chars)", &text[..end], text.chars().count()),
    }
}

fn walk(value: &Value, model: &str, path: &str) -> Option<String> {
    let object = value.as_object()?;
    for (wire, required, nested, container) in data().models.get(model)? {
        let at = if path.is_empty() {
            wire.clone()
        } else {
            format!("{path}.{wire}")
        };
        let item = match object.get(wire) {
            Some(item) if !is_empty(item) => item,
            _ if *required => return Some(at),
            _ => continue,
        };
        let nested = match nested {
            Some(nested) => nested,
            None => continue,
        };
        match container.as_str() {
            "list" => {
                if let Some(items) = item.as_array() {
                    for (i, item) in items.iter().enumerate() {
                        if let Some(found) = walk(item, nested, &format!("{at}[{i}]")) {
                            return Some(found);
                        }
                    }
                }
            }
            "map" => {
                if let Some(items) = item.as_object() {
                    for (key, item) in items {
                        if let Some(found) = walk(item, nested, &format!("{at}.{key}")) {
                            return Some(found);
                        }
                    }
                }
            }
            _ => {
                if let Some(found) = walk(item, nested, &at) {
                    return Some(found);
                }
            }
        }
    }
    None
}
