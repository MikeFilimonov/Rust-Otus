DROP TABLE IF EXISTS devices;
DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS smart_home;

CREATE TABLE smart_home(
    id serial PRIMARY KEY,
    title VARCHAR(150) UNIQUE NOT NULL
    );

CREATE TABLE rooms(
    
    id serial PRIMARY KEY,
    title VARCHAR(150) UNIQUE NOT NULL,
    smart_home_id INT NOT NULL,
    FOREIGN KEY (smart_home_id) 
    REFERENCES smart_home(id)
);

CREATE TABLE devices(
    
    id serial,
    smart_home_id INT NOT NULL,
    room_id INT NOT NULL,
    PRIMARY KEY (id, smart_home_id, room_id),
    FOREIGN KEY (smart_home_id)
        REFERENCES smart_home(id),
    FOREIGN KEY (room_id)
        REFERENCES rooms(id),
    title VARCHAR(150) UNIQUE NOT NULL
);

INSERT INTO 
    smart_home(title) 
VALUES ('home');

INSERT INTO 
    rooms(title, smart_home_id) 
VALUES 
    ('bedroom', 1),
    ('living room', 1),
    ('bath', 1),
    ('kitchen', 1);

INSERT INTO 
    devices(title, smart_home_id, room_id)
    VALUES 
    ('smart_outlet', 1, 1),
    ('smart_outlet', 1, 2),
    ('smart_outlet', 1, 3),
    ('smart_outlet', 1, 4),
    ('smart_vc', 1, 2),
    ('smart_thermometer', 1, 2);
