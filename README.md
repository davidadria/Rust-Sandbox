# Rust-Sandbox

Aprendiendo Rust desde 0

## Recursos

Toda la información del lenguaje  
[Web-Oficial](https://rust-lang.org/es/)

Ejemplos Ejecutables  
[Rust-By-Example](https://doc.rust-lang.org/rust-by-example/es/index.html)

Guía a Seguir  
[Roadmap](https://roadmap.sh/rust)

Libro Oficial  
[The Rust Programming Language](https://doc.rust-lang.org/book/)

Proyectos de Internet
[100 Days of Rust](https://github.com/LiveGray/100-Days-Of-Rust)

## Estructura

```bash
Rust-Sandbox/
├── 00-hello_world
├── 01-variables_data_types_constants
├── 02-control_flow_constructs
├── 03-functions_methods
├── 04-structs
├── 05-enums
├── 100_days_rust
│   ├── PR001-age_to_days
│   ├── PR002-finding_nemo
│   ├── PR003-barbacue
│   └── PR004-jonny_progress
├── ejercicios
│   ├── ejercicio_01_calculadora
│   ├── ejercicio_02_nave_espacial
│   ├── ejercicio_03_funciones_basicas
│   ├── ejercicio_04_rpg
│   └── ejercicio_05_reproductor_multimedia
├── .gitignore
└── README.md
```

## Contenidos

### [00 - Hello World](./00-hello_world)

Lo más básico de todos los lenguajes de programación.

### [01 - Variables, DataTypes and Constants](./01-variables_data_types_constants)

- Declaración de variables
- Tipos de dato
- Constantes
- Mutabilidad

### [02 - Control Flow Constructs](./02-control_flow_constructs)

- if / else
- if / let
- match
- for
- while
- loop
- break

### [03 - Functions and Method Syntax](./03-functions_methods)

- Declaración
- Parámetros
- Declaracion Vs Expresion
- Return

### [04 - Structs](./04-structs)

- Declaracion
- Instanciar (atajo incluido)
- Obtener valores
- Traspasar y cambiar valores
- Tuple Struct
- Unit Struct

### [05 - Enum](./05-enums)

- Declaración
- Valores
- Option
- Pattern Matching

## Ejercicios

### [Calculadora](./ejercicios/ejercicio_01_calculadora)

- Pedir datos al usuario (std::io)
- Mostrar Resultados
  - Suma
  - Resta
  - Multiplicar
  - Dividir

### [Consola Nave Espacial](./ejercicios/ejercicio_02_nave_espacial)

- Pedir datos al usuario (std::io)
- Condicionales
- Bucles

### [Funciones Básicas](./ejercicios/ejercicio_03_funciones_basicas)

- Funciones que reciben parametros
- Funciones que devuelven valor
- Operaciones matematicas simples
- Manejo simple de strings

### [RPG](./ejercicios/ejercicio_04_rpg)

- Structs
- Funciones
- Operaciones simples

### [Reproductor Multimedia](./ejercicios/ejercicio_05_reproductor_multimedia)

- Enums
- Array
- Bucle For
- Funciones
- Match Pattern

## 100 Days of Rust

### [Día 1: Convertir Años a Días](./100_days_rust/PR001-age_to_days)

- Pedir datos al usuario (std::io)
- Parsear String -> Int

### [Día 2: Buscando a Nemo](./100_days_rust/PR002-finding_nemo)

- Manejo de Strings
- Bucle For
- Uso de Flags

### [Día 3: Barbacue](./100_days_rust/PR003-barbacue)

- Manejo de Strings
- Bucle For
- Condicionales
- Array

### [Día 4: Jonny's Progress](./100_days_rust/PR004-jonny_progress)

- Arrays
- Bucle While
