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
-- Name: allergies; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.allergies (
    aid integer NOT NULL,
    name text NOT NULL,
    symptoms text NOT NULL
);


ALTER TABLE public.allergies OWNER TO postgres;

--
-- Data for Name: allergies; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.allergies (aid, name, symptoms) FROM stdin;
1	low Sugar	Feeling shaky or trembling.
2	Low Cholesterol	Changes in your mood, sleep, or  eating patterns.
3	Diabetes	Increased thirst.
4	Anaphylaxis	Rapid fall in blood pressure.
5	Fish	Vomitting and diarrhea.
\.


--
-- Name: allergies allergies_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.allergies
    ADD CONSTRAINT allergies_pkey PRIMARY KEY (aid);


--
-- PostgreSQL database dump complete
--

