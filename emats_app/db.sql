drop table if exists devices cascade;
drop table if exists entries;

create table devices(
	id varchar(128) primary key,
	dev_location varchar(150) not null,
	key varchar(512) not null,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

create table entries(
	id serial primary key,
	device_id varchar(128) not null,
	val smallint not null,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	foreign key (device_id) references devices(id)
);

insert into devices (id, dev_location, key)
	values ('ef6bd3ee-37ea-4249-8d7e-7b1f7a35e815','home', 'P06xNSaAdTNffgmu6P2Nmw9u8f uNfQduIJobxnRO6zRmRQQZfo3vZI9Yk UBGPrp4uHvMGdb8DsnfGWsrw8AjwFp6KiYh N9s2xHAywQqlyInAs87P6IezRsRW5oG8etosLl7HylSyYMHvWTzwSMVtS1ENtqhxmGaD8I0hg8vUYXPRFYpgeG4fTIfHDclxD9S4ALuNFAMemfr4wjA1E8Ik8BJOzPSYDpj5hSfOnlYuVa7eheB3lmFYK5Av2kOpijUspNSiNZFgmVonda6FRLmra6tf1sT7a3 I1UFWCBFntuhqkYTulDlCNjhdaBfa86CSAtX4N2VuBscXNC EebjOeZL wbE NV6dPRc8yJ 335aPiV7k6RCXRzml Hp0D7UHNKT5mbeRtaekzu0pkA uYAs2tR5jtwAEY hiwZ9bO1mDnod1QOEjZqfwFH7fDMUguMqpNVvEupSwTsp1hF2gQFfsIB3venvgI0Gc68Q0R4wzeJjUWxR1LyZJMmfN3');

insert into entries (device_id, val)
	values ('ef6bd3ee-37ea-4249-8d7e-7b1f7a35e815', 500);
