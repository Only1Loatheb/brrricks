# Brrricks

```mermaid
sequenceDiagram
    participant User
    participant Platform
    box Purple
    participant App
    end
    participant SessionStore

    User->>Platform: Dial *123#
    Platform->>App: /session/new
    App->>App: Process initial request
    App->>SessionStore: Store session
    SessionStore-->>App: Session stored
    App-->>Platform: First USSD screen
    Platform-->>User: Display USSD screen

    User->>Platform: Input value
    Platform->>App: /session/continue
    App->>SessionStore: Fetch session data
    SessionStore-->>App: Session data
    App->>App: Process input
    App->>SessionStore: Update session
    SessionStore-->>App: Session updated
    App-->>Platform: Second USSD screen
    Platform-->>User: Display USSD screen
```

## Process modelling

[Process builder state diagram](type_process_builder/src/process_builder_diagram.mermaid)

## Plausible use cases

[Africa's Talking API Reference](https://developers.africastalking.com/docs/ussd/handle_sessions)

[Creditswitch API Reference](https://developers.creditswitch.com/pages/ussd)

[Qrios API Reference](https://deep.qrios.com/api/doc/developer-guide/sdk/ussd)
