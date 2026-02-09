# Brrricks

## Brrricks app session flow

```mermaid
sequenceDiagram
    actor User
    participant Platform
    box Purple
        participant App
    end
    participant SessionStore
    User ->> Platform: Dial *123 #35;
    Platform ->> App: /session/new
    App ->> App: Process initial request
    App ->> SessionStore: Store session
    SessionStore -->> App: Session stored
    App -->> Platform: First USSD screen
    Platform -->> User: Display USSD screen
    User ->> Platform: Input value
    Platform ->> App: /session/continue
    App ->> SessionStore: Fetch session data
    SessionStore -->> App: Session data
    App ->> App: Process input
    App ->> SessionStore: Update session
    SessionStore -->> App: Session updated
    App -->> Platform: Input USSD screen
    Platform -->> User: Display USSD screen
    User ->> Platform: Input value
    Platform ->> App: /session/continue
    App ->> SessionStore: Fetch session data
    SessionStore -->> App: Session data
    App ->> App: Process input
    App -->> Platform: Final USSD screen
    Platform -->> User: Display USSD screen
```

## Process builder states

```mermaid
%%{ init: { 'flowchart': {'defaultRenderer': 'tidy-tree' } } }%%
flowchart TD
    classDef hidden display: none;
    style Start fill-opacity:0, stroke-opacity:0;
    Start[" "]
    FinalizedSplitProcessSubgraph:::hidden
    subgraph FinalizedSplitProcessSubgraph[" "]
        FinalizedSplitProcess[FinalizedSplitProcess]
        finalized_split_cases_final{{exhaustive?}}
    end
    FlowingSplitProcessSubgraph:::hidden
    subgraph FlowingSplitProcessSubgraph[" "]
        FlowingSplitProcess[FlowingSplitProcess]
        flowing_split_cases{{exhaustive?}}
    end
    FinalizedProcess -- "build" --> RunnableProcess
    Start -- "Entry Step" --> FlowingProcess
    FlowingProcess -- "Operation Step or FlowingProcess" --> FlowingProcess
    FlowingProcess -- "Final Step or FinalizedProcess" --> FinalizedProcess
    
    FlowingProcess -- "Split Step" --> FinalizedSplitProcess
    FinalizedSplitProcess -- "FinalizedProcess" --> finalized_split_cases_final
    finalized_split_cases_final -- "cases left" --> FinalizedSplitProcess
    finalized_split_cases_final -- "all cases covered" --> FinalizedProcess
    FinalizedSplitProcess -- "FlowingProcess" --> FlowingSplitProcess
    FlowingSplitProcess -- "FinalizedProcess or FlowingProcess" --> flowing_split_cases
    flowing_split_cases -- "cases left" --> FlowingSplitProcess
    flowing_split_cases -- "all cases covered" --> FlowingProcess
    Start ~~~ FlowingSplitProcess
    click FlowingSplitProcess "https://github.com/Only1Loatheb/brrricks/blob/master/type_process_builder/src/builder/flowing_split_process.rs"
    click FlowingProcess "https://github.com/Only1Loatheb/brrricks/blob/master/type_process_builder/src/builder/flowing_process.rs"
    click FinalizedSplitProcess "https://github.com/Only1Loatheb/brrricks/blob/master/type_process_builder/src/builder/finalized_split_process.rs"
    click FinalizedProcess "https://github.com/Only1Loatheb/brrricks/blob/master/type_process_builder/src/builder/finalized_process.rs"
    click RunnableProcess "https://github.com/Only1Loatheb/brrricks/blob/master/type_process_builder/src/builder/runnable_process.rs"
```

## Plausible use cases

[Africa's Talking API Reference](https://developers.africastalking.com/docs/ussd/handle_sessions)

[Creditswitch API Reference](https://developers.creditswitch.com/pages/ussd)

[Qrios API Reference](https://deep.qrios.com/api/doc/developer-guide/sdk/ussd)

[//]: # (todo Redirect)

[//]: # (todo ReturnFromRedirect)

[//]: # (todo Back)

[//]: # (todo ConditionalBack)
