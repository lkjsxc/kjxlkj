# Transport Security

Back: [/docs/spec/security/README.md](/docs/spec/security/README.md)

## TLS Boundary

- App server runs HTTP inside the container/service network.
- TLS termination is handled by reverse proxy in front of `kjxlkj`.

## Header Expectations

When behind a trusted reverse proxy:

- forwarded proto/header mapping MUST be configured explicitly
- secure cookie behavior MUST respect forwarded HTTPS context

## Deployment Guidance

- Do not expose plain HTTP publicly without trusted front proxy.
- Use network controls to limit direct container access.

## Related

- Deployment: [/docs/spec/architecture/deployment.md](/docs/spec/architecture/deployment.md)
- Guides: [/docs/guides/DOCKER.md](/docs/guides/DOCKER.md)
