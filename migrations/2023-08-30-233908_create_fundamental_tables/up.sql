-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY NOT NULL,
  username VARCHAR(255) UNIQUE NOT NULL,
  password VARCHAR(255) NOT NULL
);

INSERT INTO users (username, password) VALUES 
('test', '$2b$12$0ZZyUOJL.3tASromWavwTOD/uH8Wj.mjf8C2QWJ4rpTMEJHpcLTHq');

CREATE TABLE IF NOT EXISTS recipe (
  id  SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  hidden BOOLEAN DEFAULT FALSE
);

INSERT INTO recipe (user_id, name, description) VALUES 
  (1, 'Gnocchi with Creamed Spinach', 'Honestly, we can''t get enough of this super fast weeknight meal. \nGnocchi is a pillowy soft pasta that, if you aren''t keeping in your pantry, you should be. And frozen spinach is a versatile and healthy ingredient that can be used so many different ways. Besides those two, all you need for this 10-minute meal is some cream, some cheese, and a few pantry spices. \nServe it with a glass of wine and you''ve got a fancy dinner for two that tastes like it took a lot more work than it actually did.');

CREATE TABLE IF NOT EXISTS recipe_ingredient (
  id SERIAL PRIMARY KEY NOT NULL,
  recipe_id INT NOT NULL REFERENCES recipe(id) ON DELETE CASCADE ON UPDATE CASCADE,
  name VARCHAR(20) NOT NULL,
  quantity VARCHAR(10),
  measurement_type VARCHAR(10)
);

INSERT INTO recipe_ingredient (recipe_id, name, quantity, measurement_type) VALUES 
  (1, 'Kosher salt', null, null),
  (1, 'Black Pepper', null, 'ground'),
  (1, 'Gnocchi', '19', 'oz'),
  (1, 'Spinach', '9', 'oz'),
  (1, 'Heavy Cream', '1/2', 'c'),
  (1, 'Parmesan', '2', 'oz'),
  (1, 'nutmeg', '1/8', 'oz. grated'),
  (1, 'Lemon zest', null, 'garnish');

CREATE TABLE IF NOT EXISTS recipe_step (
  id SERIAL PRIMARY KEY NOT NULL,
  recipe_id INT NOT NULL REFERENCES recipe(id) ON DELETE CASCADE ON UPDATE CASCADE,
  step_number INT NOT NULL,
  step_directions TEXT NOT NULL
);

INSERT INTO recipe_step (recipe_id, step_number, step_directions) VALUES 
  (1, 1, 'In a large pot of salted water, cook gnocchi according to package directions. '),
  (1, 2, 'Meanwhile, cook cream of spinach and cream in a large, nonstick skillet over medium heat until warm and thickened, 3 to 4 minutes. Season with salt and pepper. Add Parmesan, nutmeg, and gnocchi, and stir to coat. Serve garnished with lemon zest.');
