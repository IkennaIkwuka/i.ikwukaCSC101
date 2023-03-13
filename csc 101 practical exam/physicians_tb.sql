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
-- Name: physicians; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.physicians (
    phid integer NOT NULL,
    last_name text NOT NULL,
    first_name text NOT NULL,
    e_mail text NOT NULL,
    specialization text NOT NULL
);


ALTER TABLE public.physicians OWNER TO postgres;

--
-- Data for Name: physicians; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.physicians (phid, last_name, first_name, e_mail, specialization) FROM stdin;
1	AUDU	GLORIA	g_audu@patron.org	Orthopedic
2	SEIDU	AHMED	a_seidu@patron.org	Surgery
3	GBENGA	MILDRED	m_gbenga@patron.org	Pediatrics
\.


--
-- Name: physicians physicians_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.physicians
    ADD CONSTRAINT physicians_pkey PRIMARY KEY (phid);


--
-- PostgreSQL database dump complete
--

