CREATE KEYSPACE IF NOT EXISTS dt_master WITH REPLICATION = {
  'class' : 'SimpleStrategy',
  'replication_factor' : 1 
};

USE dt_master;

# User
CREATE TYPE IF NOT EXISTS user (
  id UUID PRIMARY KEY,
  email text,
  name text,
  password text
);

# Twin Instance
CREATE TABLE IF NOT EXISTS twin (
  id UUID PRIMARY KEY, # uuid()
  name text,
  created_at timestamp # toTimestamp(now())
);

# Device

# Sensor
