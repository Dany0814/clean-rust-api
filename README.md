### clean-rust-api

##### Application structure

which may be handy for other projects.

    ├── src    # This main folder
    │   ├── domain              # In this layer you encapsulate the business rules and here you also define custom validations and errors.
    │   │   ├── models          # This folder contains the structures to encapsulate business models
    │   │   └── ...             # Here should go everything related to general business rules.
    │   ├── infrastructure      # This layer is related to frameworks, databases, among others. Changes made to this layer should not  affect the inner layers.
    │   │   ├── datastore       # This folder contains the connection to db.
    │   │   ├── router          # This folder contains the routes of the endpoints.
    │   │   └── ...             # Here goes any framework, ui, etc.
    │   ├── interface           # This layer serves as a translator between the infraestructure layer and the domain, packaging the input and output data as needed.
    │   │   ├── controller      # I / O controller.
    │   │   └── presenters      # 
    │   │   └── repository      #    
    │   ├── usecase             # This layer in which the business rules of the application are defined.
    │   │   ├── interactor      # This folder contains the interactions between the interface layer and use cases
    │   │   ├── presenter       # In this directory the functions that define the route entries are defined, these functions are implemented in the interaction directory
    │   │   └── repository      # This folder contains the application rules to be used for data access.
    └── ...     # Implementation of all required services.