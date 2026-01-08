CREATE TABLE "follow" (
 "id" serial NOT NULL PRIMARY KEY,
 "user_id" VARCHAR(50) NOT NULL,
 "target_id" VARCHAR(50) NOT NULL,
 "target_type" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);

CREATE UNIQUE INDEX uk_follow_user_target ON "follow" ("user_id","target_id","target_type");

CREATE INDEX idx_follow_user ON "follow" ("user_id","created");
CREATE INDEX idx_follow_target ON "follow" ("target_id","target_type","created");


CREATE TABLE "favorite" (
 "id" serial NOT NULL PRIMARY KEY,
 "user_id" VARCHAR(50) NOT NULL,
 "target_id" VARCHAR(50) NOT NULL,
 "target_type" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);

CREATE UNIQUE INDEX uk_favorite_user_target ON "favorite" ("user_id","target_id","target_type");

CREATE INDEX idx_favorite_user ON "favorite" ("user_id","created");
CREATE INDEX idx_favorite_target ON "favorite" ("target_id","target_type","created");


CREATE TABLE "likee" (
 "id" serial NOT NULL PRIMARY KEY,
 "user_id" VARCHAR(50) NOT NULL,
 "target_id" VARCHAR(50) NOT NULL,
 "target_type" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);

CREATE UNIQUE INDEX uk_like_user_target ON "likee" ("user_id","target_id","target_type");

CREATE INDEX idx_like_target ON "likee" ("target_id","target_type","created");
CREATE INDEX idx_like_user ON "likee" ("user_id","created");


CREATE TABLE "score" (
 "id" serial NOT NULL PRIMARY KEY,
 "user_id" VARCHAR(50) NOT NULL,
 "target_id" VARCHAR(50) NOT NULL,
 "target_type" SMALLINT NOT NULL,
 "score" bigint NOT NULL,
 "comment" Text,
 "created" bigint NOT NULL,
 "updated" bigint
);

CREATE UNIQUE INDEX uk_score_user_target ON "score" ("user_id","target_id","target_type");

CREATE INDEX idx_score_user ON "score" ("user_id","created");
CREATE INDEX idx_score_target ON "score" ("target_id","target_type","created");


CREATE TABLE "commentt" (
 "comment_id" VARCHAR(50) NOT NULL PRIMARY KEY,
 "user_id" VARCHAR(50) NOT NULL,
 "target_id" VARCHAR(50) NOT NULL,
 "target_type" SMALLINT NOT NULL,
 "content" Text NOT NULL,
 "parent_id" VARCHAR(50),
 "created" bigint NOT NULL,
 "updated" bigint
);

CREATE INDEX idx_comm_user ON "commentt" ("user_id","created");
CREATE INDEX idx_comm_target ON "commentt" ("target_id","target_type","created");
CREATE INDEX id_comm_parent ON "commentt" ("parent_id");


COMMENT ON TABLE "follow" IS '';
COMMENT ON COLUMN "follow"."id" IS '应用ID';
COMMENT ON COLUMN "follow"."user_id" IS '用户ID';
COMMENT ON COLUMN "follow"."target_id" IS '被关注对象ID';
COMMENT ON COLUMN "follow"."target_type" IS '被关注对象类型';
COMMENT ON COLUMN "follow"."created" IS '创建时间';
COMMENT ON COLUMN "follow"."updated" IS '更新时间';
COMMENT ON TABLE "favorite" IS '';
COMMENT ON COLUMN "favorite"."id" IS '应用ID';
COMMENT ON COLUMN "favorite"."user_id" IS '用户ID';
COMMENT ON COLUMN "favorite"."target_id" IS '被收藏对象ID';
COMMENT ON COLUMN "favorite"."target_type" IS '被收藏对象类型';
COMMENT ON COLUMN "favorite"."created" IS '创建时间';
COMMENT ON COLUMN "favorite"."updated" IS '更新时间';
COMMENT ON TABLE "likee" IS '点赞表';
COMMENT ON COLUMN "likee"."id" IS '应用ID';
COMMENT ON COLUMN "likee"."user_id" IS '用户ID';
COMMENT ON COLUMN "likee"."target_id" IS '被点赞对象ID';
COMMENT ON COLUMN "likee"."target_type" IS '被点赞对象类型';
COMMENT ON COLUMN "likee"."created" IS '创建时间';
COMMENT ON COLUMN "likee"."updated" IS '更新时间';
COMMENT ON TABLE "score" IS '';
COMMENT ON COLUMN "score"."id" IS '应用ID';
COMMENT ON COLUMN "score"."user_id" IS '评分人用户ID ';
COMMENT ON COLUMN "score"."target_id" IS '被评分对象ID';
COMMENT ON COLUMN "score"."target_type" IS '被评分对象类型';
COMMENT ON COLUMN "score"."score" IS '分数';
COMMENT ON COLUMN "score"."comment" IS '评分附加说明或评语';
COMMENT ON COLUMN "score"."created" IS '创建时间';
COMMENT ON COLUMN "score"."updated" IS '更新时间';
COMMENT ON TABLE "commentt" IS '';
COMMENT ON COLUMN "commentt"."comment_id" IS '应用ID';
COMMENT ON COLUMN "commentt"."user_id" IS '用户ID';
COMMENT ON COLUMN "commentt"."target_id" IS '评论对象ID';
COMMENT ON COLUMN "commentt"."target_type" IS '评论对象类型（如 post, article, video 等）';
COMMENT ON COLUMN "commentt"."content" IS '';
COMMENT ON COLUMN "commentt"."parent_id" IS '应用ID';
COMMENT ON COLUMN "commentt"."created" IS '创建时间';
COMMENT ON COLUMN "commentt"."updated" IS '更新时间';


CREATE TABLE "config" (
 "id" serial NOT NULL PRIMARY KEY,
 "app_name" VARCHAR(50) NOT NULL,
 "app_version" VARCHAR(15),
 "var_name" VARCHAR(50) NOT NULL,
 "var_value" VARCHAR(500) NOT NULL,
 "public" SMALLINT NOT NULL,
 "remark" VARCHAR(500),
 "status" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);

CREATE INDEX idx_config_app ON "config" ("app_name","created");


COMMENT ON TABLE "config" IS '常量配置表
';
COMMENT ON COLUMN "config"."id" IS '应用ID';
COMMENT ON COLUMN "config"."app_name" IS '用户ID';
COMMENT ON COLUMN "config"."app_version" IS '为空表示 支持所有版本';
COMMENT ON COLUMN "config"."var_name" IS '用户ID';
COMMENT ON COLUMN "config"."var_value" IS '用户ID';
COMMENT ON COLUMN "config"."remark" IS '';
COMMENT ON COLUMN "config"."status" IS '0 禁用  1启用';
COMMENT ON COLUMN "config"."created" IS '创建时间';
COMMENT ON COLUMN "config"."updated" IS '更新时间';


CREATE TABLE "user" (
 "user_id" VARCHAR(50) NOT NULL PRIMARY KEY,
 "username" VARCHAR(50) NOT NULL,
 "password" VARCHAR(500) NOT NULL,
 "nickname" VARCHAR(50) NOT NULL,
 "avatar" SMALLINT NOT NULL,
 "status" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);

CREATE UNIQUE INDEX uk_user ON "user" ("username");


COMMENT ON TABLE "user" IS '';
COMMENT ON COLUMN "user"."user_id" IS '用户ID';
COMMENT ON COLUMN "user"."username" IS '用名登陆名';
COMMENT ON COLUMN "user"."password" IS '用名密码';
COMMENT ON COLUMN "user"."nickname" IS '用名呢称';
COMMENT ON COLUMN "user"."status" IS '0 禁用  1启用';
COMMENT ON COLUMN "user"."created" IS '创建时间';
COMMENT ON COLUMN "user"."updated" IS '更新时间';
