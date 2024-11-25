# Proyecto 1 de Gráficas por computadora | Raycasting

## Escape the Death Star

---

## Video demostrativo

[![Watch the video](https://img.youtube.com/vi/zxQbFAqLbIw/0.jpg)](https://youtu.be/zxQbFAqLbIw)

---

### Descripción

Escape the Death Star es un emocionante juego 3D tipo laberinto desarrollado en Rust. El jugador debe navegar a través de un laberinto en tiempo limitado, enfrentándose a enemigos y utilizando un minimapa para encontrar la salida antes de que el tiempo se agote. El juego incluye múltiples niveles, cada uno con un diseño de laberinto único, seleccionable desde la pantalla de bienvenida.

---

## Características principales

- Gráficos en 3D: Renderizado en tiempo real con un enfoque retro basado en raycasting.
- Múltiples niveles: El jugador puede elegir entre 3 niveles diferentes, cada uno con su propio laberinto.
- Minimapa: Incluye un minimapa en la esquina superior derecha para facilitar la navegación.
- Pantallas interactivas: Pantalla de bienvenida con selección de niveles, además de pantallas de victoria y derrota.
- Audio inmersivo: Música y efectos de sonido en momentos clave del juego.

---

## Requisitos del sistema/Dependencias

Para ejecutar el juego, asegúrate de tener lo siguiente instalado:

- Rust (versión 1.70 o superior). Puedes instalarlo desde rustup.rs.
- Cargo: Administrador de paquetes incluido con Rust.

El proyecto utiliza las siguientes dependencias de Rust:

- minifb: Para crear ventanas y manejar entradas de teclado.
- nalgebra_glm: Para matemáticas 2D/3D y transformaciones geométricas.
- once_cell: Para inicializar estáticamente texturas y recursos.
- rodio: Para manejar audio.
- image: Para cargar y procesar texturas.

---

## Cómo ejecutar el proyecto

Clonar el repositorio:
```bash
Copy code
git clone <https://github.com/nel-eleven11/Graficas_Proy1>
cd Graficas_Proy1
```

Construir el proyecto:
```bash
Copy code
cargo build --release
```

Ejecutar el proyecto:
```bash
Copy code --release
cargo run
```
---

## Controles del juego

Pantalla de bienvenida:
* Flecha Arriba/Abajo: Navegar entre niveles.
* Enter: Seleccionar nivel y comenzar el juego.

Dentro del juego:
* W: Mover hacia adelante.
* S: Mover hacia atrás.
* A: Mover hacia la izquierda.
* D: Mover hacia la derecha.
* ← y → (mouse): Rotar la vista.
* M: Alternar entre modo 2D y 3D.
* Escape: Salir del juego.
