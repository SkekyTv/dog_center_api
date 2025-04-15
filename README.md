# Dog center

Dog center purpose is to display an interface for `DogTrainer` to manage their `TrainingSessions` and their `Customers`.

`Customers` will be allowed to create an account and manage their dog profile.

`DogTrainer` (admin) will display training sessions in a planning.

`Customers` will ask to join sessions. `DogTrainer` have to confirm them.

`Customers` and `DogTrainer` will be notified with email/sms.

`DogTrainer` can set up `TrainingSessions` by themself with `Customers`.

## Models

```mermaid

---
title: DogCenter diagram class
---
classDiagram

class Skills {
    id
    label
    customersNote
    TrainerNote
  }

class Dogs {
    id
    name
    birthdate
    races
    weigth
    img
  }
  Dogs : +age() number

class Customers {
    id
    name
    birthdate
    email
    pdw
    img
  }
  Customers : +age() number

class TrainingSessions {
    id
    date
    label
    adresse
    img
  }

class Trainer {
    id
    name
  }

  Customers --> Dogs : owned
  Trainer --> Customers : trained
  TrainingSessions --> Customers : participate
  Trainer --> TrainingSessions : animate
  Dogs --> Skills : learned


```

## Project architecture

`DogCenterApi` is based on clean architecture pattern.

```css

src/
├── main.rs
├── entities/
│ └── user.rs
├── use_cases/
│ └── user_service.rs
├── interfaces/
│ └── http/
│   ├── handlers.rs
│   └── routes.rs
├── infra/
│ └── db/
│   └── user_repository.rs
├── repositories/
│   └── user_repository.rs
└── shared/
  └── error.rs
```
