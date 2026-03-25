# Integrations

_Last updated: YYYY-MM-DD_

## Overview

This document describes external services and third-party integrations used by this project.

## Active Integrations

### [Integration Name]

**Purpose:** [What this integration does]

**Provider:** [Service provider name]

**Documentation:** [Link to provider docs]

**Configuration:**
- Environment variables required: `INTEGRATION_API_KEY`, `INTEGRATION_URL`
- Config file: `config/integrations/[name].json`

**Usage:**
```typescript
// Example usage
import { integration } from '@/lib/integrations/name';

await integration.doSomething();
```

**Rate Limits:** [Any rate limiting considerations]

**Error Handling:** [How errors from this integration are handled]

---

### [Another Integration]

**Purpose:** [What this integration does]

**Provider:** [Service provider name]

**Documentation:** [Link to provider docs]

**Configuration:**
- Environment variables required: `ANOTHER_API_KEY`
- Config file: N/A

---

## Webhooks

### Incoming Webhooks

| Endpoint | Source | Purpose |
|----------|--------|---------|
| `/api/webhooks/[name]` | [Provider] | [What it handles] |

### Outgoing Webhooks

| Event | Destination | Purpose |
|-------|-------------|---------|
| `event.name` | [URL/Service] | [What it triggers] |

---

## Authentication

### OAuth Providers

| Provider | Scopes | Callback URL |
|----------|--------|--------------|
| [Provider] | `scope1`, `scope2` | `/api/auth/callback/[provider]` |

### API Keys

| Service | Environment Variable | Rotation Policy |
|---------|---------------------|-----------------|
| [Service] | `SERVICE_API_KEY` | [Rotation frequency] |

---

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `INTEGRATION_API_KEY` | Yes | API key for [integration] |
| `INTEGRATION_URL` | No | Custom URL (defaults to production) |

---

## Testing

### Mocking Integrations

[Describe how integrations are mocked in tests]

### Sandbox/Test Environments

| Integration | Test Environment | Test Credentials |
|-------------|------------------|------------------|
| [Integration] | [Sandbox URL] | See `.env.test` |

---

## Troubleshooting

### Common Issues

**Issue:** [Description of common problem]
**Solution:** [How to resolve]

**Issue:** [Another common problem]
**Solution:** [How to resolve]

---

## Related Documents

- `docs/technical/auth.md` — Authentication details
- `docs/technical/deployment.md` — Environment configuration
