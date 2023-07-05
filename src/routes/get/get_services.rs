use actix_web::get;
use serde_json::json;

#[get("/client/v4/user")]
pub async fn handle_user() -> String {
    "done".to_string()
}

#[get("/client/v4/memberships")]
pub async fn handle_memberships() -> String {
    json!({
      "result": [
        {
          "id": "d1060f04981741679e4423705ee70e3f",
          "status": "accepted",
          "api_access_enabled": true,
          "permissions": {
            "integration": {
              "read": true,
              "edit": true,
              "install": true
            },
            "waitingroom": {
              "read": true,
              "edit": true
            },
            "magic": {
              "read": true,
              "edit": true
            },
            "organization": {
              "read": true,
              "edit": true
            },
            "waf": {
              "read": true,
              "edit": true
            },
            "dex": {
              "read": true,
              "edit": true
            },
            "analytics": {
              "read": true,
              "edit": false
            },
            "zone_settings": {
              "read": true,
              "edit": true
            },
            "dns_records": {
              "read": true,
              "edit": true
            },
            "worker": {
              "read": true,
              "edit": true
            },
            "zone_versioning": {
              "read": true,
              "edit": true
            },
            "ssl": {
              "read": true,
              "edit": true
            },
            "zaraz": {
              "read": true,
              "edit": true,
              "publish": true
            },
            "logs": {
              "read": true,
              "edit": true
            },
            "billing": {
              "read": true,
              "edit": true
            },
            "fbm": {
              "read": true,
              "edit": true
            },
            "fbm_acc": {
              "read": false,
              "edit": true
            },
            "http_applications": {
              "read": true,
              "edit": true
            },
            "lb": {
              "read": true,
              "edit": true
            },
            "blocks": {
              "read": true,
              "edit": true
            },
            "stream": {
              "read": true,
              "edit": true
            },
            "teams": {
              "read": true,
              "edit": true,
              "pii": true,
              "report": true
            },
            "healthchecks": {
              "read": true,
              "edit": true
            },
            "web3": {
              "read": true,
              "edit": true
            },
            "access": {
              "read": true,
              "edit": true
            },
            "image": {
              "read": true,
              "edit": true
            },
            "dash_sso": {
              "read": true,
              "edit": true
            },
            "zone": {
              "read": true,
              "edit": true
            },
            "subscription": {
              "read": true,
              "edit": true
            },
            "app": {
              "read": false,
              "edit": true
            },
            "cache_purge": {
              "read": false,
              "edit": true
            },
            "auditlogs": {
              "read": true,
              "edit": false
            },
            "member": {
              "read": true,
              "edit": true
            },
            "legal": {
              "read": true,
              "edit": true
            }
          },
          "roles": [
            "Super Administrator - All Privileges"
          ],
          "account": {
            "id": "398803b74bcdb1b454434669bc634190",
            "name": "CHANGE HERE ACCOUNT NAME",
            "settings": {
              "enforce_twofactor": false,
              "access_approval_expiry": null
            }
          },
          "policies": []
        }
      ],
      "result_info": {
        "page": 1,
        "per_page": 20,
        "total_pages": 1,
        "count": 2,
        "total_count": 2
      },
      "success": true,
      "errors": [],
      "messages": []
    })
    .to_string()
}

#[get("/client/v4/accounts/{accounts}/workers/subdomain")]
pub async fn handle_subdomain() -> String {
    //read_json("./src/defaultResponses/get_subdomain.json")
    json!({
      "result": {
        "subdomain": "CHOOSE_YOUR_SUBDOMAIN_HERE"
      },
      "success": true,
      "errors": [],
      "messages": []
    })
    .to_string()
}

#[get("/client/v4/accounts/{accounts}/workers/services/{services}")]
pub async fn handle_accounts_services() -> String {
    json!({
      "result": {
        "id": "CHANGE ID HERE",
        "default_environment": {
          "environment": "production",
          "created_on": "2023-04-11T09:04:26.21742Z",
          "modified_on": "2023-06-01T16:37:26.89231Z",
          "script": {
            "created_on": "2023-04-11T09:04:26.21742Z",
            "modified_on": "2023-06-01T16:37:26.89231Z",
            "id": "hello",
            "tag": "0c2ca8c2d4d44b2490074d7a979899e1",
            "tags": [],
            "deployment_id": "",
            "tail_consumers": null,
            "logpush": false,
            "etag": "f1b79de5c4b1af431ffbd80dcc8883429003af5359704900dd8ee6860aa904f3",
            "handlers": [
              "fetch"
            ],
            "last_deployed_from": "wrangler",
            "compatibility_date": "2023-04-11",
            "usage_model": "bundled"
          }
        },
        "created_on": "2023-04-11T09:04:26.21742Z",
        "modified_on": "2023-06-01T16:37:26.89231Z",
        "usage_model": "",
        "environments": [
          {
            "environment": "production",
            "created_on": "2023-04-11T09:04:26.21742Z",
            "modified_on": "2023-06-01T16:37:26.89231Z"
          }
        ]
      },
      "success": true,
      "errors": [],
      "messages": []
    })
    .to_string()
}
