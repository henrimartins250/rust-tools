use rusqlite::{params, Connection, Result};

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

use chrono::{NaiveDateTime};

pub fn pet_shop_db() -> Result<()>  {


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
    pub struct Cliente {
        cpf: String,
        nome: String,
        telefone: String,
        endereco: String,
    }

    pub struct Animal {
        codigo_animal: i32,
        nome: String,
        tipo: String,
        cpf_dono: String,
    }

    pub struct ServicoAnimal {
        codigo_servico: i32,
        cpf_cliente: String,
        cpf_funcionario: String,
        codigo_animal: i32,
        descricao: String,
        preco: f32,
        data: NaiveDateTime,
    }

    pub struct Funcionario {
        cpf: String,
        nome: String,
        endereco: String,
        telefone: String,
        funcao: String,
    }

    pub struct Produto {
        codigo_produto: i32,
        nome: String,
        descricao: String,
        preco: f32,
        quantidade: i32,
    }

    let conn = Connection::open_in_memory()?;

    conn.execute(
        "
            PRAGMA foreign_keys = ON;

create table cliente(
    cpf INTEGER PRIMARY KEY,
    nome varchar(100) not null,
);

create table animal(
    id INTEGER PRIMARY KEY,
    nome varchar(100)  NOT NULL,
    especie varchar(100) NOT NULL,
    cpf_cliente char(11) NOT NULL,
    FOREIGN KEY (cpf_cliente) REFERENCES cliente(cpf)
);

create table funcionario(
    cpf INTEGER PRIMARY KEY NOT NULL,
    nome varchar(100) NOT NULL,
    endereco varchar(100) NOT NULL,
    telefone varchar(20) NOT NULL,
    funcao varchar(100) NOT NULL,

);

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



create view Boletim_Servicos as
    select
        cliente.nome,
        animal.nome,
        servico_animal.descricao,
        funcionario.nome,
        servico_animal.preco,
        servico_animal.data
            from servico_animal
                join cliente on servico_animal.cpf_cliente = cliente.cpf
                join animal on servico_animal.id_animal = animal.id_animal
                join funcionario on servico_animal.cpf_funcionario = funcionario.cpf;
        ",
        (), // empty list of parameters.
    )?;

    let mut stmt = conn.prepare(
        "SELECT name, type 
         FROM sqlite_master 
         WHERE type IN ('table', 'view') 
         AND name NOT LIKE 'sqlite_%'"
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

    Ok(())
}

pub fn rh_db() {

}

pub fn academic_db() {

}