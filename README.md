1. Agregar las tablas de tables.sql a mysql
* Puedes hacerlo con docker, en el archivo src/db_insert.rs al final viene un comentario con un ejemplo para correr docker

2. Agregar las variables de entorno para mysql
export DB_USER='<db_user>'
export DB_PASSWORD='<db_password>'
export DB_HOST='<db_host>'
export DB_PORT='<db_port>'
export DB_DATABASE='<db_database>'

> Con los datos de tu database


3. Ejecutar el script db_inserts para agregar los datos base a la base de datos

