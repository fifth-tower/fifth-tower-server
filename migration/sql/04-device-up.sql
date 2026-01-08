CREATE TABLE "user_device" (
 "id" serial NOT NULL PRIMARY KEY,
 "user_id" VARCHAR(50) NOT NULL,
 "device_code" VARCHAR(500) NOT NULL,
 "ip" VARCHAR(50),
 "status" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);

CREATE INDEX idx_user_device_user ON "user_device" ("user_id");


COMMENT ON TABLE "user_device" IS '';
COMMENT ON COLUMN "user_device"."id" IS '';
COMMENT ON COLUMN "user_device"."user_id" IS '用户ID';
COMMENT ON COLUMN "user_device"."device_code" IS '设备码';
COMMENT ON COLUMN "user_device"."ip" IS '设备码';
COMMENT ON COLUMN "user_device"."created" IS '创建时间';
COMMENT ON COLUMN "user_device"."updated" IS '更新时间';
