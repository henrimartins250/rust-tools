use rusqlite::{Connection, Result, params};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

pub fn init() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person?);
    }
    Ok(())
}

use chrono::NaiveDateTime;

pub fn pet_shop_db() -> Result<()> {
    /*
        Atividade 01
    Considere um sistema de Pet Shop com as tabelas Cliente, Animal, Servico_Animal, Funcionario e Produto, onde:

    Cliente contém CPF, nome, telefone e endereço.

    Animal contém código do animal, nome, tipo e CPF do cliente dono.

    Servico_Animal registra os serviços prestados, com código do serviço, CPF do cliente, CPF do funcionário, código do animal, descrição do serviço, preço e data.

    Funcionario contém CPF, nome, endereço, telefone e função.

    Produto contém código do produto, nome, descrição, preço e quantidade em estoque.

    Crie uma VIEW chamada Boletim_Servicos que exiba os seguintes dados para cada serviço prestado:
    Nome do cliente
    Nome do animal
    Descrição do serviço
    Nome do funcionário responsável
    Preço do serviço
    Data do serviço
    A VIEW deve relacionar as tabelas para mostrar o histórico completo dos serviços prestados a cada animal.
    */
    let conn = Connection::open_in_memory()?;

    conn.execute_batch(
        "
            PRAGMA foreign_keys = ON;

create table cliente(
    cpf INTEGER PRIMARY KEY,
    nome varchar(100) not null,
    telefone varchar(50) not null,
    endereco varchar(150) not null
);

INSERT INTO cliente (cpf, nome, telefone, endereco) VALUES
(1, 'Alice', '555-0101', 'Rua A, 123'),
(2, 'Bob', '555-0202', 'Rua B, 456'),
(3, 'Carol', '555-0303', 'Rua C, 789'),
(4, 'David', '555-0404', 'Rua D, 101'),
(5, 'Eva', '555-0505', 'Rua E, 202');

create table animal(
    id INTEGER PRIMARY KEY,
    nome varchar(100)  NOT NULL,
    especie varchar(100) NOT NULL,
    cpf_cliente char(11) NOT NULL,
    FOREIGN KEY (cpf_cliente) REFERENCES cliente(cpf)
);

INSERT INTO animal (id, nome, especie, cpf_cliente) VALUES
(1, 'Rex', 'Cachorro', 1),
(2, 'Miau', 'Gato', 2),
(3, 'Bolt', 'Cachorro', 3),
(4, 'Nina', 'Gato', 4),
(5, 'Toby', 'Cachorro', 5);


create table funcionario(
    cpf INTEGER PRIMARY KEY NOT NULL,
    nome varchar(100) NOT NULL,
    endereco varchar(100) NOT NULL,
    telefone varchar(20) NOT NULL,
    funcao varchar(100) NOT NULL

);

INSERT INTO funcionario (cpf, nome, endereco, telefone, funcao) VALUES
(100, 'F1', 'End 1', '555-1001', 'Vendedor'),
(101, 'F2', 'End 2', '555-1002', 'Vendedor'),
(102, 'F3', 'End 3', '555-1003', 'Veterinario');

create table servico_animal(
    id_Servico INTEGER PRIMARY KEY,
    cpf_cliente CHAR(11) NOT NULL,
    cpf_funcionario CHAR(11) NOT NULL,
    id_animal int NOT NULL,
    descricao VARCHAR(100) NOT NULL,
    preco DECIMAL(10,2) NOT NULL,
    data TEXT NOT NULL, -- entregue pela api em formatio ISO 8601 YYYY-MM-DD
    FOREIGN KEY (cpf_cliente) REFERENCES cliente(cpf),
    FOREIGN KEY (cpf_funcionario) REFERENCES funcionario(cpf),
    FOREIGN KEY (id_animal) REFERENCES animal(id)
);

INSERT INTO servico_animal (id_servico, cpf_cliente, cpf_funcionario, id_animal, descricao, preco, data) VALUES
(1, 1, 102, 1, 'Banho', 30.0, '2025-10-01'),
(2, 2, 102, 2, 'Tosa', 40.0, '2025-10-02');

create table produto (
id INTEGER PRIMARY KEY,
nome varchar(150) not null,
descricao varchar(150) not null,
preco decimal(10, 2) not null,
estoque integer not null
);

INSERT INTO produto (id, nome, descricao, preco, estoque) VALUES
(1, 'Racao', 'Racao premium', 50.0, 10),
(2, 'Coleira', 'Coleira nylon', 20.0, 5),
(3, 'Brinquedo', 'Bola de borracha', 15.0, 8);

create table compra(
id integer primary key,
cpf_cliente char(11) not null,
cpf_funcionario char(11) not null,
total decimal(10,2) not null,
data date not null,
foreign key (cpf_cliente) references cliente(cpf),
foreign key(cpf_funcionario) references funcionario(cpf)
);

INSERT INTO compra (id, cpf_cliente, cpf_funcionario, total, data) VALUES
(1, 1, 100, 70.0, '2025-10-01'),
(2, 2, 101, 35.0, '2025-10-02'),
(3, 3, 102, 15.0, '2025-10-03');


create table compra_produto (
id_compra integer not null,
id_produto integer not null,
primary key (id_compra, id_produto),
foreign key (id_compra) references compra(id),
foreign key (id_produto) references produto(id)
);

INSERT INTO compra_produto (id_compra, id_produto) VALUES
(1, 1),
(1, 2),
(2, 2),
(3, 3);





create view Boletim_Servicos as
    select
        cliente.nome as cliente_nome,
        animal.nome as animal_nome,
        servico_animal.descricao as servico_descricao,
        funcionario.nome as funcionario_nome,
        servico_animal.preco as servico_preco,
        servico_animal.data as servico_data
            from servico_animal
                join cliente on servico_animal.cpf_cliente = cliente.cpf
                join animal on servico_animal.id_animal = animal.id
                join funcionario on servico_animal.cpf_funcionario = funcionario.cpf;
 
create view relatorio_compras as
        select     
        cliente.nome as cliente_nome,
        funcionario.nome as funcionario_nome,
        compra.id as compra_id,
          produto.nome as nome_produto,
          compra.total as valor_total,
        compra.data as compra_data
          from compra
            join cliente on compra.cpf_cliente = cliente.cpf
            join funcionario on compra.cpf_funcionario = funcionario.cpf
            join compra_produto on compra_produto.id_compra = compra.id
            join produto on compra_produto.id_produto = produto.id;
        ",
    )?;

    let mut stmt = conn.prepare(
        "
         SELECT name, type 
         FROM sqlite_master 
         WHERE type IN ('table', 'view') 
         AND name NOT LIKE 'sqlite_%'
",
    )?;

    let rows = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let object_type: String = row.get(1)?;
        Ok((name, object_type))
    })?;

    println!("Tables and Views:");
    for row in rows {
        let (name, object_type) = row?;
        println!(" - {} ({})", name, object_type);
    }

    let mut stmt1 = conn.prepare(
          "SELECT cliente_nome, animal_nome, servico_descricao, funcionario_nome, servico_preco, servico_data
     FROM Boletim_Servicos",
    )?;

    let rows = stmt1.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, String>(5)?,
        ))
    })?;

    println!("Boletim_Servicos:");
    for row in rows {
        let (cliente, animal, servico, funcionario, preco, data) = row?;
        println!(
            "Cliente: {}, Animal: {}, Servico: {}, Funcionario: {}, Preco: {}, Data: {}",
            cliente, animal, servico, funcionario, preco, data
        );
    }

    // Query relatorio_compras
    let mut stmt2 = conn.prepare(
        "SELECT cliente_nome, funcionario_nome, compra_id, nome_produto, valor_total, compra_data
     FROM relatorio_compras",
    )?;

    let rows2 = stmt2.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, String>(5)?,
        ))
    })?;

    println!("\nRelatorio_Compras:");
    for row in rows2 {
        let (cliente, funcionario, compra_id, produto, total, data) = row?;
        println!(
            "Cliente: {}, Funcionario: {}, Compra: {}, Produto: {}, Total: {}, Data: {}",
            cliente, funcionario, compra_id, produto, total, data
        );
    }

    Ok(())
}

pub fn rh_db() {}

pub fn academic_db() {}

