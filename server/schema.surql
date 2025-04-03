REMOVE DATABASE manger_dog;
DEFINE DATABASE manger_dog;

DEFINE TABLE users SCHEMAFULL;
DEFINE FIELD email ON TABLE users TYPE string;
DEFINE FIELD username ON TABLE users TYPE string;
DEFINE FIELD password ON TABLE users TYPE string;
DEFINE FIELD registration_date ON TABLE users TYPE datetime DEFAULT time::now();
DEFINE FIELD selected_preference ON TABLE users TYPE option<record<preferences>>;
DEFINE INDEX email_idx ON TABLE users COLUMNS email UNIQUE;

DEFINE TABLE accounts SCHEMAFULL;
DEFINE FIELD title ON TABLE accounts TYPE string;
DEFINE FIELD user_id ON TABLE accounts TYPE option<record<users>>;
DEFINE FIELD group_id ON TABLE accounts TYPE option<record<groups>>;
DEFINE FIELD currency_id ON TABLE accounts TYPE record<currencies>;
DEFINE FIELD balance ON TABLE accounts TYPE number;

DEFINE TABLE groups SCHEMAFULL;
DEFINE FIELD owner_id ON TABLE groups TYPE record<users>;
DEFINE FIELD metadata ON TABLE groups TYPE record<metadata>;

DEFINE TABLE group_members SCHEMAFULL;
DEFINE FIELD group_id ON TABLE group_members TYPE record<groups>;
DEFINE FIELD role_id ON TABLE group_members TYPE record<group_roles>;
DEFINE FIELD member_id ON TABLE group_members TYPE record<users>;

DEFINE TABLE group_roles SCHEMAFULL;
DEFINE FIELD identifier ON TABLE group_roles TYPE string;
INSERT INTO group_roles (identifier) VALUES ('owner'), ('member'), ('readonly');

DEFINE TABLE invitations SCHEMAFULL;
DEFINE FIELD sender_id ON TABLE invitations TYPE record<users>;
DEFINE FIELD receiver_id ON TABLE invitations TYPE record<users>;
DEFINE FIELD group_id ON TABLE invitations TYPE record<groups>;
DEFINE FIELD group_role_id ON TABLE invitations TYPE record<group_roles>;
DEFINE FIELD message ON TABLE invitations TYPE string;

DEFINE TABLE tags SCHEMAFULL;
DEFINE FIELD user_id ON TABLE tags TYPE record<users>;
DEFINE FIELD metadata_id ON TABLE tags TYPE record<metadata>;

DEFINE TABLE financial_goals SCHEMAFULL;
DEFINE FIELD user_id ON TABLE financial_goals TYPE record<users>;
DEFINE FIELD currency_id ON TABLE financial_goals TYPE record<currencies>;
DEFINE FIELD target_income ON TABLE financial_goals TYPE number;
DEFINE FIELD start_date ON TABLE financial_goals TYPE datetime;
DEFINE FIELD end_date ON TABLE financial_goals TYPE datetime;
DEFINE FIELD metadata_id ON TABLE financial_goals TYPE record<metadata>;

DEFINE TABLE transactions SCHEMAFULL;
DEFINE FIELD account_id ON TABLE transactions TYPE record<accounts>;
DEFINE FIELD amount ON TABLE transactions TYPE number;
DEFINE FIELD date ON TABLE transactions TYPE datetime;
DEFINE FIELD metadata_id ON TABLE transactions TYPE record<metadata>;

DEFINE TABLE transfers SCHEMAFULL;
DEFINE FIELD account_from ON TABLE transfers TYPE record<accounts>;
DEFINE FIELD account_to ON TABLE transfers TYPE record<accounts>;
DEFINE FIELD amount_from ON TABLE transfers TYPE number;
DEFINE FIELD amount_to ON TABLE transfers TYPE number;
DEFINE FIELD conversion_rate ON TABLE transfers TYPE option<number>;
DEFINE FIELD fee ON TABLE transfers TYPE number DEFAULT 0;
DEFINE FIELD metadata_id ON TABLE transfers TYPE record<metadata>;

DEFINE TABLE stable_incomes SCHEMAFULL;
DEFINE FIELD user_id ON TABLE stable_incomes TYPE record<users>;
DEFINE FIELD currency_id ON TABLE stable_incomes TYPE record<currencies>;
DEFINE FIELD amount_per_month ON TABLE stable_incomes TYPE number;
DEFINE FIELD start_date ON TABLE stable_incomes TYPE datetime;
DEFINE FIELD end_date ON TABLE stable_incomes TYPE datetime;
DEFINE FIELD last_update_date ON TABLE stable_incomes TYPE datetime;
DEFINE FIELD metadata_id ON TABLE stable_incomes TYPE record<metadata>;

DEFINE TABLE stable_income_incomes SCHEMAFULL;
DEFINE FIELD stable_income_id ON TABLE stable_income_incomes TYPE record<stable_incomes>;
DEFINE FIELD transaction_id ON TABLE stable_income_incomes TYPE record<transactions>;

DEFINE TABLE loans SCHEMAFULL;
DEFINE FIELD user_id ON TABLE loans TYPE record<users>;
DEFINE FIELD currency_id ON TABLE loans TYPE record<currencies>;
DEFINE FIELD principal_amount ON TABLE loans TYPE number;
DEFINE FIELD interest_rate ON TABLE loans TYPE number;
DEFINE FIELD start_date ON TABLE loans TYPE datetime;
DEFINE FIELD end_date ON TABLE loans TYPE option<datetime>;
DEFINE FIELD interest_rate_type ON TABLE loans TYPE record<interest_rate_types>;
DEFINE FIELD compounding_frequency ON TABLE loans TYPE record<compounding_frequencies>;
DEFINE FIELD metadata_id ON TABLE loans TYPE record<metadata>;

DEFINE TABLE interest_rate_types SCHEMAFULL;
DEFINE FIELD identifier ON TABLE interest_rate_types TYPE string;
INSERT INTO interest_rate_types (identifier) VALUES ('simple'), ('compound');

DEFINE TABLE compounding_frequencies SCHEMAFULL;
DEFINE FIELD identifier ON TABLE compounding_frequencies TYPE string;
INSERT INTO compounding_frequencies (identifier) VALUES ('annually'), ('semi_annually'), ('quarterly'), ('monthly'), ('daily');

DEFINE TABLE loan_payments SCHEMAFULL;
DEFINE FIELD loan_id ON TABLE loan_payments TYPE record<loans>;
DEFINE FIELD transaction_id ON TABLE loan_payments TYPE record<transactions>;

-- DEFINE TABLE loan_payment_types SCHEMAFULL;
-- DEFINE FIELD identifier ON TABLE loan_payment_types TYPE string;
-- INSERT INTO loan_payment_types (identifier) VALUES ('scheduled'), ('extra'), ('minimum');

DEFINE TABLE investments SCHEMAFULL;
DEFINE FIELD user_id ON TABLE investments TYPE record<users>;
DEFINE FIELD currency_id ON TABLE investments TYPE record<currencies>;
DEFINE FIELD type ON TABLE investments TYPE record<investment_types>;
DEFINE FIELD compounding_frequency ON TABLE investments TYPE record<compounding_frequencies>;
DEFINE FIELD principal_amount ON TABLE investments TYPE number;
DEFINE FIELD start_date ON TABLE investments TYPE datetime;
DEFINE FIELD expected_end_date ON TABLE investments TYPE datetime;
DEFINE FIELD end_date ON TABLE investments TYPE datetime;
DEFINE FIELD risk_level ON TABLE investments TYPE record<risk_levels>;
DEFINE FIELD expected_return ON TABLE investments TYPE number;
DEFINE FIELD metadata_id ON TABLE investments TYPE record<metadata>;

DEFINE TABLE investment_types SCHEMAFULL;
DEFINE FIELD identifier ON TABLE investment_types TYPE string;
INSERT INTO investment_types (identifier) VALUES ('stocks'), ('bonds'), ('real_estate'), ('mutual_funds'), ('etfs'), ('cryptocurrency'), ('precious_metals'), ('commodities'), ('p2p_lending');

DEFINE TABLE risk_levels SCHEMAFULL;
DEFINE FIELD identifier ON TABLE risk_levels TYPE string;
INSERT INTO interest_rate_types (identifier) VALUES ('low'), ('moderate'), ('high'), ('very_high');

DEFINE TABLE investment_returns SCHEMAFULL;
DEFINE FIELD investment_id ON TABLE investment_returns TYPE record<investments>;
DEFINE FIELD account_id ON TABLE investment_returns TYPE record<investments>;
DEFINE FIELD date ON TABLE investment_returns TYPE datetime;
DEFINE FIELD amount ON TABLE investment_returns TYPE number;
DEFINE FIELD metadata_id ON TABLE investment_returns TYPE record<metadata>;

DEFINE TABLE metadata SCHEMAFULL;
DEFINE FIELD title ON TABLE metadata TYPE string;
DEFINE FIELD description ON TABLE metadata TYPE string;

DEFINE TABLE metadata_tags SCHEMAFULL;
DEFINE FIELD metadata_id ON TABLE metadata_tags TYPE record<metadata>;
DEFINE FIELD tag_id ON TABLE metadata_tags TYPE record<tags>;
DEFINE FIELD exception ON TABLE metadata_tags TYPE bool;
DEFINE INDEX metadata_tag_exception_idx ON TABLE metadata_tags COLUMNS exception;

DEFINE TABLE tag_groups SCHEMAFULL;
DEFINE FIELD metadata_id ON TABLE tag_groups TYPE option<record<metadata>>;

DEFINE TABLE tag_group_tags SCHEMAFULL;
DEFINE FIELD tag_group_id ON TABLE tag_group_tags TYPE record<tag_groups>;
DEFINE FIELD tag_id ON TABLE tag_group_tags TYPE record<tags>;

DEFINE TABLE metadata_tag_groups SCHEMAFULL;
DEFINE FIELD metadata_id ON TABLE metadata_tag_groups TYPE record<metadata>;
DEFINE FIELD tag_group_id ON TABLE metadata_tag_groups TYPE record<tag_groups>;

DEFINE TABLE currencies SCHEMAFULL;
DEFINE FIELD identifier ON TABLE currencies TYPE string;
DEFINE INDEX identifier_idx ON TABLE currencies COLUMNS identifier UNIQUE;
INSERT INTO currencies (identifier) VALUES ("USD"), ("EUR"), ("GBP"), ("JPY"), ("AUD"), ("CAD"), ("CHF"), ("CNY"), ("INR"), ("BRL"), ("BTC"), ("ETH"), ("USDT"), ("USDC"), ("RUB");

DEFINE TABLE preferences SCHEMAFULL;
DEFINE FIELD user_id ON TABLE preferences TYPE record<users>;
DEFINE FIELD default_currency_id ON TABLE preferences TYPE record<currencies>;
-- DEFINE FIELD theme ON TABLE preferences TYPE record<color_palettes>;
DEFINE FIELD language ON TABLE preferences TYPE record<languages>;

DEFINE TABLE languages SCHEMAFULL;
DEFINE FIELD identifier ON TABLE languages TYPE string;
DEFINE INDEX identifier_idx ON TABLE languages FIELDS identifier UNIQUE;
INSERT INTO languages (identifier) VALUES ("EN"), ("RU");

DEFINE TABLE auto_distributions SCHEMAFULL;
DEFINE FIELD ratio ON TABLE auto_distributions TYPE number;
DEFINE FIELD account_id ON TABLE auto_distributions TYPE record<accounts>;
DEFINE FIELD metadata_id ON TABLE auto_distributions TYPE record<metadata>;
DEFINE FIELD record_metadata_id ON TABLE auto_distributions TYPE record<metadata>;

DEFINE TABLE financial_goal_auto_distributions SCHEMAFULL;
DEFINE FIELD financial_goal_id ON TABLE financial_goal_auto_distributions TYPE record<financial_goals>;
DEFINE FIELD auto_distribution_id ON TABLE financial_goal_auto_distributions TYPE record<auto_distributions>;
DEFINE INDEX idx ON TABLE financial_goal_auto_distributions COLUMNS auto_distribution_id, financial_goal_id UNIQUE;

DEFINE TABLE stable_income_auto_distributions SCHEMAFULL;
DEFINE FIELD stable_income_id ON TABLE stable_income_auto_distributions TYPE record<stable_incomes>;
DEFINE FIELD auto_distribution_id ON TABLE stable_income_auto_distributions TYPE record<auto_distributions>;
DEFINE INDEX idx ON TABLE stable_income_auto_distributions COLUMNS auto_distribution_id, stable_income_id UNIQUE;

DEFINE TABLE loan_auto_distributions SCHEMAFULL;
DEFINE FIELD loan_id ON TABLE loan_auto_distributions TYPE record<loans>;
DEFINE FIELD auto_distribution_id ON TABLE loan_auto_distributions TYPE record<auto_distributions>;
DEFINE INDEX idx ON TABLE loan_auto_distributions COLUMNS auto_distribution_id, loan_id UNIQUE;

DEFINE TABLE investment_auto_distributions SCHEMAFULL;
DEFINE FIELD investment_id ON TABLE investment_auto_distributions TYPE record<investments>;
DEFINE FIELD auto_distribution_id ON TABLE investment_auto_distributions TYPE record<auto_distributions>;
DEFINE INDEX idx ON TABLE investment_auto_distributions COLUMNS auto_distribution_id, investment_id UNIQUE;

DEFINE TABLE transfers_auto_distributions SCHEMAFULL;
DEFINE FIELD metadata_id ON TABLE transfers_auto_distributions TYPE record<metadata>;
DEFINE FIELD account_to ON TABLE transfers_auto_distributions TYPE record<accounts>;
DEFINE FIELD auto_distribution_id ON TABLE transfers_auto_distributions TYPE record<auto_distributions>;

DEFINE TABLE transactions_auto_distributions SCHEMAFULL;
DEFINE FIELD auto_distribution_id ON TABLE transactions_auto_distributions TYPE record<auto_distributions>;

DEFINE TABLE sessions SCHEMAFULL;
DEFINE FIELD session_id ON TABLE sessions TYPE string;
DEFINE FIELD user_id ON TABLE sessions TYPE record<users>;
DEFINE FIELD expiration_date ON TABLE sessions TYPE datetime;
DEFINE INDEX user_id_idx ON TABLE sessions COLUMNS user_id;
DEFINE INDEX session_id_idx ON TABLE sessions COLUMNS session_id;
DEFINE INDEX session_id_user_id_idx ON TABLE sessions COLUMNS session_id, user_id UNIQUE;
DEFINE INDEX expiration_date_idx ON TABLE sessions COLUMNS expiration_date;

-- TODO: color palettes
-- TODO: color palettes in preferences
-- TODO: custom color palettes (same table as color palettes, but with no owner)
-- TODO: keymapping

DEFINE TABLE financial_goals_investments SCHEMAFULL;
DEFINE FIELD financial_goal_id ON TABLE financial_goals_investments TYPE record<financial_goals>;
DEFINE FIELD date ON TABLE financial_goals_investments TYPE datetime;
DEFINE FIELD amount ON TABLE financial_goals_investments TYPE number;
DEFINE FIELD metadata_id ON TABLE financial_goals_investments TYPE record<metadata>;