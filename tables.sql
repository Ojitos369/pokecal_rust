-- MySQL

CREATE TABLE `tipos` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `nombre` varchar(255) NOT NULL,
    
    constraint pk_tipos primary key (id)
);

-- da_doble, recibe_doble, da_mitad, recibe_mitad, da_nada, recibe_nada
CREATE TABLE `da_doble` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `tipo` int(11) NOT NULL,
    `da_a` int(11) NOT NULL,

    constraint pk_da_doble primary key (id),
    constraint fk_da_doble_tipo foreign key (tipo) references tipos(id),
    constraint fk_da_doble_da_a foreign key (da_a) references tipos(id)
);

CREATE TABLE `recibe_doble` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `tipo` int(11) NOT NULL,
    `recibe_de` int(11) NOT NULL,

    constraint pk_recibe_doble primary key (id),
    constraint fk_recibe_doble_tipo foreign key (tipo) references tipos(id),
    constraint fk_recibe_doble_recibe_de foreign key (recibe_de) references tipos(id)
);

CREATE TABLE `da_mitad` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `tipo` int(11) NOT NULL,
    `da_a` int(11) NOT NULL,

    constraint pk_da_mitad primary key (id),
    constraint fk_da_mitad_tipo foreign key (tipo) references tipos(id),
    constraint fk_da_mitad_da_a foreign key (da_a) references tipos(id)
);

CREATE TABLE `recibe_mitad` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `tipo` int(11) NOT NULL,
    `recibe_de` int(11) NOT NULL,

    constraint pk_recibe_mitad primary key (id),
    constraint fk_recibe_mitad_tipo foreign key (tipo) references tipos(id),
    constraint fk_recibe_mitad_recibe_de foreign key (recibe_de) references tipos(id)
);

CREATE TABLE `da_nada` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `tipo` int(11) NOT NULL,
    `da_a` int(11) NOT NULL,

    constraint pk_da_nada primary key (id),
    constraint fk_da_nada_tipo foreign key (tipo) references tipos(id),
    constraint fk_da_nada_da_a foreign key (da_a) references tipos(id)
);

CREATE TABLE `recibe_nada` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `tipo` int(11) NOT NULL,
    `recibe_de` int(11) NOT NULL,

    constraint pk_recibe_nada primary key (id),
    constraint fk_recibe_nada_tipo foreign key (tipo) references tipos(id),
    constraint fk_recibe_nada_recibe_de foreign key (recibe_de) references tipos(id)
);


-- drop table recibe_nada;
-- drop table da_nada;
-- drop table recibe_mitad;
-- drop table da_mitad;
-- drop table recibe_doble;
-- drop table da_doble;
-- drop table tipos;
