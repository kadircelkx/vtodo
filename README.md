# vtodo

![LICENSE](https://img.shields.io/badge/license-GPL--3.0-blue)
![LANG](https://img.shields.io/badge/language-rust-orange)

**Vtodo** is a Vi-like todo editor designed for the terminal. It offers a fast and intuitive interface, making task management efficient and enjoyable.

## Features

- **Blazingly fast**: Built for speed to keep up with your workflow.
- **Easy to use with commands**: Utilize simple commands for seamless task management.
- **Colorful TUI**: Enhances user experience with vibrant colors for easy visualization.
- **Indexed Todos**: Each todo is assigned an index number for quick reference.

## Installation
### Source
For manual installation (recommended), follow these steps:

1. Clone this repository:

```bash
git clone https://github.com/kadircelkx/vtodo.git
cd vtodo
```

2. Build with Cargo:

```bash
cargo build --release
```

3. Copy the executable to your system's binary directory:

```bash
cp ./target/release/vtodo /usr/local/bin/
```

## Usage

Simply type `vtodo` in your terminal to start. You'll see an input prompt like this:


```bash
Todos
--------------------
> 
```

**Vtodo** operates using commands. Here's how you can use it:

### Add

To add a new todo, type `add my first todo` at the input prompt. This will create and display the new todo.

### Done

Mark a todo as done by typing `done 1`, where 1 is the id of the todo you want to mark as done. The todo will be highlighted in green.

### Undone

To mark a todo as undone, use the command `undone 1`, where 1 is the id of the todo you want to mark as undone. The todo will be highlighted in red.

### Edit

Edit a todo by typing `edit 1`, where 1 is the id of the todo you want to edit. Then, a new input prompt will appear where you can enter the new message for the todo. The ID will remain unchanged.

### Delete

Delete a todo with the command `delete 1`, where 1 is the id of the todo you want to delete. The todo will be removed from the list.

## License

Please refer to [LICENSE](./LICENSE) for more information.

Feel free to explore Vtodo and manage your tasks efficiently in the terminal!