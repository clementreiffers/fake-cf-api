use actix_web::post;
use serde_json::json;

#[post("/client/v4/accounts/{accounts}/workers/scripts/{script}/subdomain")]
pub async fn handle_accounts_scripts() -> String {
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
