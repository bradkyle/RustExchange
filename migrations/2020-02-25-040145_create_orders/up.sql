-- Your SQL goes here

CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  userid INTEGER NOT NULL REFERENCES users,
  instrumentid INTEGER NOT NULL REFERENCES instruments,
  side TEXT NOT NULL,
  ord_status TEXT NOT NULL,
  ord_type TEXT NOT NULL,
  exec_inst TEXT NOT NULL,
  time_in_force TEXT NOT NULL,
  initial_qty integer NOT NULL,
  leaves_qty integer NOT NULL,
  price real NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
