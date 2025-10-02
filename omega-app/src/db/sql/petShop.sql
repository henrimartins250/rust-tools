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

create table produto (
id INTEGER PRIMARY KEY,
nome varchar(150) not null,
descricao varchar(150) not null,
preco decimal(10, 2) not null,
estoque integer not null,
);

create table compra(
id integer primary key,
cpf_cliente char(11) not null,
cpf_funcionario char(11) not null,
total decimal(10,2) not null,
data date not null,
foreign key (cpf_cliente) references cliente(cpf),
foreign key(cpf_funcionario) references funcionario(cpf)
);

create table compra_produto (
id_compra integer not null,
id_produto integer not null,
primary key (compraid, prodid),
foreign key (references compra(id),
foreign key (id_produto) references produto(id)
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

      create view relatorio_compras as
        select     
          cliente.nome,
          funcionario.nome,
          compra.id,
          produto.nome as nome_produto,
          compra.total as valor_total,
          compra.data
          from compra
            join cliente on compra.cpf_cliente = cliente.cpf
            join funcionario on compra.cpf_funcionario = funcionario.cpf
            join compra_produto on compra_produto.id_compra = compra.id
            join produto on compraproduto.prodid = produto.idprod;
