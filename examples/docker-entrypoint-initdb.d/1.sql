-- Create the HR database schema
CREATE SCHEMA IF NOT EXISTS hr;

-- Create employees table
CREATE TABLE hr.employees (
                              employee_id SERIAL PRIMARY KEY,
                              first_name VARCHAR(50) NOT NULL,
                              last_name VARCHAR(50) NOT NULL,
                              email VARCHAR(100) UNIQUE NOT NULL,
                              phone_number VARCHAR(20),
                              hire_date DATE NOT NULL,
                              job_id VARCHAR(10) NOT NULL,
                              salary NUMERIC(10, 2) CHECK (salary > 0),
                              department_id INT,
                              manager_id INT,
                              FOREIGN KEY (manager_id) REFERENCES hr.employees(employee_id) ON DELETE SET NULL
);

-- Create departments table
CREATE TABLE hr.departments (
                                department_id SERIAL PRIMARY KEY,
                                department_name VARCHAR(50) NOT NULL,
                                manager_id INT,
                                FOREIGN KEY (manager_id) REFERENCES hr.employees(employee_id) ON DELETE SET NULL
);

-- Insert sample data into departments
INSERT INTO hr.departments (department_name, manager_id) VALUES
                                                             ('Engineering', NULL),
                                                             ('Human Resources', NULL),
                                                             ('Sales', NULL),
                                                             ('Marketing', NULL);

-- Insert sample data into employees
INSERT INTO hr.employees (first_name, last_name, email, phone_number, hire_date, job_id, salary, department_id, manager_id) VALUES
                                                                                                                                ('John', 'Doe', 'johndoe@example.com', '123-456-7890', '2022-01-15', 'ENG01', 70000, 1, NULL),
                                                                                                                                ('Jane', 'Smith', 'janesmith@example.com', '123-456-7891', '2022-02-01', 'HR01', 60000, 2, NULL),
                                                                                                                                ('Alice', 'Johnson', 'alicej@example.com', '123-456-7892', '2021-11-25', 'SALES01', 55000, 3, 1),
                                                                                                                                ('Bob', 'Brown', 'bobb@example.com', '123-456-7893', '2023-01-10', 'MKT01', 50000, 4, NULL),
                                                                                                                                ('Charlie', 'Black', 'charlieb@example.com', '123-456-7894', '2022-03-20', 'ENG02', 75000, 1, 1),
                                                                                                                                ('Diana', 'White', 'dianaw@example.com', '123-456-7895', '2021-09-15', 'ENG03', 72000, 1, 1),
                                                                                                                                ('Ethan', 'Green', 'ethang@example.com', '123-456-7896', '2021-08-10', 'FIN01', 65000, 5, 2),
                                                                                                                                ('Fiona', 'Harris', 'fionah@example.com', '123-456-7897', '2020-07-12', 'ENG02', 80000, 1, 3),
                                                                                                                                ('George', 'Clark', 'georgec@example.com', '123-456-7898', '2022-10-05', 'ENG01', 72000, 1, 4),
                                                                                                                                ('Hannah', 'Adams', 'hannaa@example.com', '123-456-7899', '2022-12-20', 'HR02', 62000, 2, NULL),
                                                                                                                                ('Ian', 'Moore', 'ianm@example.com', '123-456-7800', '2023-01-05', 'ENG03', 73000, 1, 5),
                                                                                                                                ('Jenna', 'Taylor', 'jennat@example.com', '123-456-7801', '2021-06-30', 'SALES02', 56000, 3, 1),
                                                                                                                                ('Kyle', 'Anderson', 'kylea@example.com', '123-456-7802', '2020-11-15', 'ENG02', 76000, 1, 6),
                                                                                                                                ('Laura', 'Thomas', 'laurat@example.com', '123-456-7803', '2022-09-25', 'HR01', 63000, 2, NULL),
                                                                                                                                ('Mike', 'Martinez', 'mikem@example.com', '123-456-7804', '2022-04-01', 'FIN02', 67000, 5, 2),
                                                                                                                                ('Nina', 'Lopez', 'ninal@example.com', '123-456-7805', '2022-06-15', 'MKT02', 52000, 4, 2),
                                                                                                                                ('Oscar', 'Gonzalez', 'oscarg@example.com', '123-456-7806', '2023-01-20', 'ENG03', 79000, 1, 7),
                                                                                                                                ('Paula', 'Wilson', 'paulaw@example.com', '123-456-7807', '2022-03-05', 'ENG01', 73000, 1, 3),
                                                                                                                                ('Quinn', 'King', 'quinnk@example.com', '123-456-7808', '2021-05-10', 'SALES03', 57000, 3, 1),
                                                                                                                                ('Riley', 'Scott', 'rileys@example.com', '123-456-7809', '2021-12-10', 'ENG03', 78000, 1, 8),
                                                                                                                                ('Sophia', 'Hill', 'sophiah@example.com', '123-456-7810', '2022-11-01', 'HR01', 64000, 2, NULL),
                                                                                                                                ('Tom', 'Young', 'tomy@example.com', '123-456-7811', '2023-02-01', 'FIN03', 69000, 5, 2),
                                                                                                                                ('Uma', 'Allen', 'umaa@example.com', '123-456-7812', '2020-03-15', 'MKT01', 53000, 4, 3),
                                                                                                                                ('Victor', 'Wright', 'victorw@example.com', '123-456-7813', '2022-08-20', 'ENG02', 77000, 1, 9),
                                                                                                                                ('Wendy', 'Lopez', 'wendyl@example.com', '123-456-7814', '2021-04-18', 'ENG01', 74000, 1, 5),
                                                                                                                                ('Xander', 'Perez', 'xanderp@example.com', '123-456-7815', '2021-02-11', 'HR03', 65000, 2, NULL),
                                                                                                                                ('Yara', 'Long', 'yaral@example.com', '123-456-7816', '2023-01-12', 'FIN02', 71000, 5, 4),
                                                                                                                                ('Zane', 'Watson', 'zanew@example.com', '123-456-7817', '2022-06-25', 'SALES01', 58000, 3, 2),
                                                                                                                                ('Amy', 'Clark', 'amyc@example.com', '123-456-7818', '2023-03-15', 'ENG01', 76000, 1, 10),
                                                                                                                                ('Bryan', 'Taylor', 'bryant@example.com', '123-456-7819', '2022-05-15', 'HR02', 66000, 2, NULL),
                                                                                                                                ('Clara', 'Adams', 'clarac@example.com', '123-456-7820', '2020-12-15', 'ENG03', 80000, 1, 3),
                                                                                                                                ('Derek', 'Moore', 'derekmo@example.com', '123-456-7821', '2021-01-20', 'MKT01', 54000, 4, 2),
                                                                                                                                ('Emily', 'Hill', 'emilyh@example.com', '123-456-7822', '2021-10-25', 'SALES03', 59000, 3, 2),
                                                                                                                                ('Felix', 'Young', 'felixy@example.com', '123-456-7823', '2023-04-05', 'ENG01', 77000, 1, 5),
                                                                                                                                ('Grace', 'Allen', 'gracea@example.com', '123-456-7824', '2023-05-10', 'HR01', 67000, 2, NULL),
                                                                                                                                ('Harry', 'King', 'harryk@example.com', '123-456-7825', '2023-06-15', 'ENG02', 81000, 1, 7),
                                                                                                                                ('Isla', 'Scott', 'islas@example.com', '123-456-7826', '2023-07-01', 'HR02', 68000, 2, NULL),
                                                                                                                                ('Jack', 'Hill', 'jackh@example.com', '123-456-7827', '2023-08-10', 'MKT02', 55000, 4, 1),
                                                                                                                                ('Kara', 'Lopez', 'karal@example.com', '123-456-7828', '2023-09-15', 'SALES01', 60000, 3, 4),
                                                                                                                                ('Liam', 'Wilson', 'liamw@example.com', '123-456-7829', '2023-10-01', 'ENG02', 82000, 1, 8),
                                                                                                                                ('Mia', 'Gonzalez', 'miag@example.com', '123-456-7830', '2023-11-10', 'HR03', 69000, 2, NULL),
                                                                                                                                ('Noah', 'Martinez', 'noahm@example.com', '123-456-7831', '2023-12-15', 'FIN01', 72000, 5, 6),
                                                                                                                                ('Olivia', 'Perez', 'oliviap@example.com', '123-456-7832', '2024-01-15', 'MKT01', 56000, 4, 2),
                                                                                                                                ('Parker', 'Adams', 'parkera@example.com', '123-456-7833', '2024-02-01', 'SALES02', 61000, 3, 1),
                                                                                                                                ('Quentin', 'Clark', 'quentinc@example.com', '123-456-7834', '2024-03-10', 'ENG03', 83000, 1, 4),
                                                                                                                                ('Rachel', 'King', 'rachelk@example.com', '123-456-7835', '2024-04-05', 'HR01', 70000, 2, NULL),
                                                                                                                                ('Sam', 'Taylor', 'samt@example.com', '123-456-7836', '2024-05-10', 'ENG01', 78000, 1, 3),
                                                                                                                                ('Tina', 'Watson', 'tinaw@example.com', '123-456-7837', '2024-06-15', 'HR02', 71000, 2, NULL),
                                                                                                                                ('Umar', 'Brown', 'umarb@example.com', '123-456-7838', '2024-07-01', 'ENG02', 84000, 1, 5),
                                                                                                                                ('Violet', 'Hill', 'violeth@example.com', '123-456-7839', '2024-08-10', 'SALES01', 62000, 3, 6),
                                                                                                                                ('Wade', 'Young', 'wadey@example.com', '123-456-7840', '2024-09-15', 'ENG03', 85000, 1, 7),
                                                                                                                                ('Xena', 'Martinez', 'xenam@example.com', '123-456-7841', '2024-10-01', 'HR03', 73000, 2, NULL),
                                                                                                                                ('Yusuf', 'Gonzalez', 'yusufg@example.com', '123-456-7842', '2024-11-10', 'FIN02', 74000, 5, 8),
                                                                                                                                ('Zara', 'Lopez', 'zaral@example.com', '123-456-7843', '2024-12-15', 'MKT02', 58000, 4, 4),
                                                                                                                                ('Aiden', 'Clark', 'aidenc@example.com', '123-456-7844', '2025-01-10', 'SALES03', 63000, 3, 9),
                                                                                                                                ('Bella', 'Young', 'bellay@example.com', '123-456-7845', '2025-02-15', 'ENG01', 79000, 1, 10),
                                                                                                                                ('Carter', 'Taylor', 'cartert@example.com', '123-456-7846', '2025-03-01', 'HR01', 74000, 2, NULL),
                                                                                                                                ('Daisy', 'Adams', 'daisya@example.com', '123-456-7847', '2025-04-10', 'ENG02', 86000, 1, 3),
                                                                                                                                ('Eli', 'King', 'elik@example.com', '123-456-7848', '2025-05-15', 'HR02', 75000, 2, NULL),
                                                                                                                                ('Faith', 'Scott', 'faiths@example.com', '123-456-7849', '2025-06-01', 'SALES02', 64000, 3, 1),
                                                                                                                                ('Gabriel', 'Perez', 'gabrielp@example.com', '123-456-7850', '2025-07-15', 'ENG03', 87000, 1, 5),
                                                                                                                                ('Hailey', 'Wilson', 'haileyw@example.com', '123-456-7851', '2025-08-10', 'HR03', 76000, 2, NULL),
                                                                                                                                ('Isaac', 'Lopez', 'isaacl@example.com', '123-456-7852', '2025-09-25', 'MKT01', 59000, 4, 7),
                                                                                                                                ('Julia', 'Brown', 'juliab@example.com', '123-456-7853', '2025-10-10', 'FIN03', 77000, 5, 6),
                                                                                                                                ('Kevin', 'Taylor', 'kevint@example.com', '123-456-7854', '2025-11-05', 'ENG01', 80000, 1, 9),
                                                                                                                                ('Lydia', 'Young', 'lydiay@example.com', '123-456-7855', '2025-12-15', 'HR01', 78000, 2, NULL),
                                                                                                                                ('Mason', 'Adams', 'masona@example.com', '123-456-7856', '2026-01-10', 'ENG02', 89000, 1, 2),
                                                                                                                                ('Nora', 'Hill', 'norah@example.com', '123-456-7857', '2026-02-01', 'SALES01', 67000, 3, 4),
                                                                                                                                ('Oliver', 'Watson', 'oliverw@example.com', '123-456-7858', '2026-03-10', 'ENG03', 91000, 1, 5),
                                                                                                                                ('Penelope', 'Gonzalez', 'penelopeg@example.com', '123-456-7859', '2026-04-15', 'HR02', 79000, 2, NULL),
                                                                                                                                ('Quincy', 'Brown', 'quincyb@example.com', '123-456-7860', '2026-05-01', 'FIN01', 81000, 5, 6),
                                                                                                                                ('Ruth', 'Martinez', 'ruthm@example.com', '123-456-7861', '2026-06-15', 'MKT01', 61000, 4, 3),
                                                                                                                                ('Sean', 'Perez', 'seanp@example.com', '123-456-7862', '2026-07-05', 'SALES02', 68000, 3, 7),
                                                                                                                                ('Tessa', 'Clark', 'tessac@example.com', '123-456-7863', '2026-08-01', 'ENG01', 82000, 1, 8),
                                                                                                                                ('Ursula', 'Taylor', 'ursulat@example.com', '123-456-7864', '2026-09-10', 'HR03', 80000, 2, NULL),
                                                                                                                                ('Victor', 'Lopez', 'victorl@example.com', '123-456-7865', '2026-10-25', 'ENG02', 92000, 1, 1),
                                                                                                                                ('Willa', 'Hill', 'willah@example.com', '123-456-7866', '2026-11-15', 'SALES03', 69000, 3, 4),
                                                                                                                                ('Xavier', 'Watson', 'xavierw@example.com', '123-456-7867', '2026-12-10', 'MKT02', 64000, 4, 2),
                                                                                                                                ('Yasmine', 'King', 'yasminek@example.com', '123-456-7868', '2027-01-15', 'HR01', 82000, 2, NULL),
                                                                                                                                ('Zack', 'Gonzalez', 'zackg@example.com', '123-456-7869', '2027-02-01', 'FIN02', 83000, 5, 9);

-- Update manager_id for departments
UPDATE hr.departments
SET manager_id = (SELECT employee_id FROM hr.employees WHERE first_name = 'John' AND last_name = 'Doe')
WHERE department_name = 'Engineering';

UPDATE hr.departments
SET manager_id = (SELECT employee_id FROM hr.employees WHERE first_name = 'Jane' AND last_name = 'Smith')
WHERE department_name = 'Human Resources';
