-- Reversal is valid while all user IDs still fit PostgreSQL INTEGER.
BEGIN;

ALTER TABLE workspaces DROP CONSTRAINT workspaces_created_by_fkey;
ALTER TABLE workspace_members DROP CONSTRAINT workspace_members_user_id_fkey;
ALTER TABLE subscriptions DROP CONSTRAINT subscriptions_user_id_fkey;
ALTER TABLE purchased_products DROP CONSTRAINT purchased_products_user_id_fkey;
ALTER TABLE invoices DROP CONSTRAINT invoices_user_id_fkey;
ALTER TABLE usage DROP CONSTRAINT usage_user_id_fkey;
ALTER TABLE transform DROP CONSTRAINT transform_owner_user_id_fkey;
ALTER TABLE transform_grants DROP CONSTRAINT transform_grants_grantee_user_id_fkey;
ALTER TABLE transform_grants DROP CONSTRAINT transform_grants_granted_by_fkey;

ALTER TABLE workspaces ALTER COLUMN created_by TYPE INTEGER;
ALTER TABLE workspace_members ALTER COLUMN user_id TYPE INTEGER;
ALTER TABLE subscriptions ALTER COLUMN user_id TYPE INTEGER;
ALTER TABLE purchased_products ALTER COLUMN user_id TYPE INTEGER;
ALTER TABLE invoices ALTER COLUMN user_id TYPE INTEGER;
ALTER TABLE usage ALTER COLUMN user_id TYPE INTEGER;
ALTER TABLE transform ALTER COLUMN owner_user_id TYPE INTEGER;
ALTER TABLE transform_grants ALTER COLUMN grantee_user_id TYPE INTEGER;
ALTER TABLE transform_grants ALTER COLUMN granted_by TYPE INTEGER;
ALTER TABLE users ALTER COLUMN user_id TYPE INTEGER;
ALTER SEQUENCE users_user_id_seq AS INTEGER;

ALTER TABLE workspaces
  ADD CONSTRAINT workspaces_created_by_fkey FOREIGN KEY (created_by) REFERENCES users(user_id);
ALTER TABLE workspace_members
  ADD CONSTRAINT workspace_members_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE;
ALTER TABLE subscriptions
  ADD CONSTRAINT subscriptions_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE;
ALTER TABLE purchased_products
  ADD CONSTRAINT purchased_products_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE;
ALTER TABLE invoices
  ADD CONSTRAINT invoices_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE;
ALTER TABLE usage
  ADD CONSTRAINT usage_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE;
ALTER TABLE transform
  ADD CONSTRAINT transform_owner_user_id_fkey FOREIGN KEY (owner_user_id) REFERENCES users(user_id);
ALTER TABLE transform_grants
  ADD CONSTRAINT transform_grants_grantee_user_id_fkey FOREIGN KEY (grantee_user_id) REFERENCES users(user_id) ON DELETE CASCADE;
ALTER TABLE transform_grants
  ADD CONSTRAINT transform_grants_granted_by_fkey FOREIGN KEY (granted_by) REFERENCES users(user_id);

COMMIT;
