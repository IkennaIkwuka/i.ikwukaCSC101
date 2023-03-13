--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: patients; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.patients (
    pid integer NOT NULL,
    last_name text NOT NULL,
    first_name text NOT NULL,
    e_mail text NOT NULL,
    date_of_birth text NOT NULL,
    phid integer NOT NULL,
    aid integer NOT NULL
);


ALTER TABLE public.patients OWNER TO postgres;

--
-- Data for Name: patients; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.patients (pid, last_name, first_name, e_mail, date_of_birth, phid, aid) FROM stdin;
1	AGADA	SIMON	a_simon@gmail.com	0	2	3
2	FAGBEMI	TINA	f_tina@gmail.com	0	3	1
3	DALEGI	VALERIE	d_valerie@gmail.com	0	1	2
4	SALAMI	SAMUEL	s_samuel@gmail.com	0	2	5
5	OGHENERO	FEJI	o_feji@gmail.com	0	3	2
6	MUSTAPHA	KABIR	m_kabir@gmail.com	0	2	4
7	ALOKWE	JANE	a_jane@gmail.com	0	3	1
8	MAKAMAN	ALI	m_ali@gmail.com	0	1	3
9	OKONKWO	CHISOM	o_chisom@gmail.com	0	1	5
10	EZE	AGATHA	e_agatha_gmail.com	0	2	1
\.


--
-- Name: patients patients_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.patients
    ADD CONSTRAINT patients_pkey PRIMARY KEY (pid);


--
-- PostgreSQL database dump complete
--

