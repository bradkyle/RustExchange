CREATE TABLE favorites (
       "user" INTEGER REFERENCES users ON DELETE CASCADE,
       snack INTEGER REFERENCES snacks ON DELETE CASCADE,
       PRIMARY KEY ("user", snack)
);
