CREATE KEYSPACE IF NOT EXISTS dt_master WITH REPLICATION = {
  'class' : 'SimpleStrategy',
  'replication_factor' : 1 
};

USE dt_master;

# User
-- CREATE TYPE IF NOT EXISTS user (
--   id UUID,
--   email text,
--   name text,
--   password text
-- );

CREATE TABLE IF NOT EXISTS user (
  id text,
  email text,
  name text,
  password text,
  PRIMARY KEY (email)
);

# Twin Instance
CREATE TABLE IF NOT EXISTS twin (
  id UUID PRIMARY KEY, # uuid()
  name text,
  created_at timestamp # toTimestamp(now())
);

# Structure
# id, name, list of devices

# Device
# id, name, list of sensors

# Sensor
# id, name, type of data

# Sensor data
# id, value, timestamp, created_at
