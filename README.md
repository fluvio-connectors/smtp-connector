# Fluvio SMTP Connector
Fluvio community Simple Mail Transfer Protocol (SMTP)

## Sink Connector
Reads from Fluvio topic and writes to SMTP.

### Configuration
| Option              | default  | type           | description                                                                                                    |
|:--------------------|:---------|:---------      |:---------------------------------------------------------------------------------------------------------------|
| host                | -        | String         | SMTP server                                                                                                    |
| port                | -        | Number         | SMTP server port - 25 typically for plaintext (if trusted network or 465 submission w/ auth)                   |
| user                | -        | String         | Username for plaintext login - must be over TLS - e.g. STARTTLS over 25 or directly over TLS port              |
| password            | -        | String         | Password for plaintext login - must be over TLS                                                                |
| dangerous_cert      | false    | String         | DANGEROUS: Upon development / debugging skip TLS cert verify - true (default false)                            |

### Usage Example

See [config-example.yaml](config-example.yaml) for an example reflecting the above.

Run connector locally using `cdk` tool (from root directory or any sub-directory):
```bash
fluvio install cdk

cdk deploy start --config config-example.yaml

cdk deploy list # to see the status
cdk deploy log my-smtp-connector # to see connector's logs
```

## License
 
- * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 
### Contribution
 
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.



