# Dummy-api 

A dummy api for creating tours(why? don't ask).

## build 
```bash 
cargo build --release
```

## run 
```bash 
cargo run --release
```

## run migrations 
```bash 
cargo install diesel_cli
diesel migration run
```

## API samples
```REST 
GET / 
list of tours
```

```REST 
POST /new 
body={
    "name": "tour1",
    "src": "ir",
    "dst": "au",
    "total_days": 1
}

```

```REST 
DELETE /{id}
```

```REST 
PUT /{id}
body={
    "name": "tour3",
    "src": "ir",
    "dst": "au",
    "total_days": 1
}
```
