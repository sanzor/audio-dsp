CREATE TABLE transform_tickets (
  ticket_id    BIGSERIAL    PRIMARY KEY,
  transform_id BIGINT       NOT NULL REFERENCES transforms(transform_id) ON DELETE CASCADE,
  issued_by    BIGINT       NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  source_code  TEXT         NOT NULL,
  status       TEXT         NOT NULL CHECK (status IN ('processing', 'failed', 'successful')),
  created_at   TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX idx_transform_tickets_transform_id ON transform_tickets(transform_id);
CREATE INDEX idx_transform_tickets_issued_by ON transform_tickets(issued_by);
CREATE INDEX idx_transform_tickets_status ON transform_tickets(status);

CREATE TABLE transform_resources (
  resource_id  BIGSERIAL    PRIMARY KEY,
  ticket_id    BIGINT       NOT NULL UNIQUE REFERENCES transform_tickets(ticket_id) ON DELETE CASCADE,
  created_at   TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX idx_transform_resources_ticket_id ON transform_resources(ticket_id);
