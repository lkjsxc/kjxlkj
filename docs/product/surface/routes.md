# Route Surface Contract

## Read Endpoints

- `GET /healthz` -> `200` with body `ok`.
- `GET /v1/records` -> `200` with JSON array sorted by `id`.
- `GET /v1/records/{id}` -> `200` with record JSON or `404`.

## Write Endpoints

- `PUT /v1/records/{id}` -> `200` on update, `201` on create.
- `DELETE /v1/records/{id}` -> `204` on delete, `404` when absent.

## Content Type

All JSON responses use `application/json` except `/healthz`.
