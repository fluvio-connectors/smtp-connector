# Fluvio SMTP Connector
Fluvio community Simple Mail Transfer Protocol (SMTP)

## Sink Connector
Reads from Fluvio topic and writes to SMTP.

### Configuration
| Option              | default  | type           | description                                                                                                    |
|:--------------------|:---------|:---------      |:---------------------------------------------------------------------------------------------------------------|
| host                | -        | String         | SMTP server                                                                                                    |
| port                | -        | Number         | SMTP server port - (465 submission w/ auth)                   |
| user                | -        | String         | Username for login - must be over TLS - e.g. STARTTLS over 25 or directly over TLS port              |
| password            | -        | String         | Password for login - must be over TLS                                                                |
| explicit_tls        | false    | bool           | Require Explicit TLS e.g. STARTTLS over plaintext SMTP port                                     |
| implicit_tls        | false    | bool           | Require Implicit TLS by ensuring the SMTP session is using TLS at all times                                    |

**Note**: Implicit TLS always takes credence over Explicit TLS (STARTTLS) and generally one should use implicit TLS if available.

### Usage Example

See [config-example.yaml](config-example.yaml) for an example reflecting the above.

Run connector locally using `cdk` tool (from root directory or any sub-directory):
```bash
fluvio install cdk

cdk deploy start --config config-example.yaml

cdk deploy list # to see the status
cdk deploy log my-smtp-connector # to see connector's logs
```

## Adding records

The JSON to use to send an e-mail via SMTP is:

```json
{
 "subject": "Testing",
 "body": "test",
 "from":
   {"name": "From Testing",
    "address": "from_address@test.com"},
 "to":
   {"name": "To Name",
    "address": "to_address@test.com"}
}
```

## License
 
- * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 
### Contribution
 
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.



