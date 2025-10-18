-- Initial schema for ctrlsys

-- Timers table
CREATE TABLE timers (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    duration_seconds INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    status TEXT NOT NULL CHECK (status IN ('pending', 'running', 'completed', 'cancelled'))
);

CREATE INDEX idx_timers_status ON timers(status);
CREATE INDEX idx_timers_expires_at ON timers(expires_at) WHERE status = 'running';

-- Locations table
CREATE TABLE locations (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    timezone TEXT NOT NULL,
    latitude REAL,
    longitude REAL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_locations_name ON locations(name);

-- Tasks table
CREATE TABLE tasks (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL CHECK (status IN ('todo', 'in_progress', 'completed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    total_time_seconds INT NOT NULL DEFAULT 0
);

CREATE INDEX idx_tasks_status ON tasks(status);

-- Project templates table
CREATE TABLE project_templates (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    template_data JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_templates_name ON project_templates(name);

-- Managed databases table
CREATE TABLE managed_databases (
    id UUID PRIMARY KEY,
    db_name TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    owner TEXT,
    notes TEXT
);

CREATE INDEX idx_managed_databases_name ON managed_databases(db_name);
