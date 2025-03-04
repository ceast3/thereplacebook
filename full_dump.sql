--
-- PostgreSQL database dump
--

-- Dumped from database version 16.3
-- Dumped by pg_dump version 16.8 (Homebrew)

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
-- Name: matches; Type: TABLE; Schema: public; Owner: radmin
--

CREATE TABLE public.matches (
    id integer NOT NULL,
    winner_id integer,
    loser_id integer,
    match_time timestamp without time zone DEFAULT now()
);


ALTER TABLE public.matches OWNER TO radmin;

--
-- Name: matches_id_seq; Type: SEQUENCE; Schema: public; Owner: radmin
--

CREATE SEQUENCE public.matches_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.matches_id_seq OWNER TO radmin;

--
-- Name: matches_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: radmin
--

ALTER SEQUENCE public.matches_id_seq OWNED BY public.matches.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: radmin
--

CREATE TABLE public.users (
    id integer NOT NULL,
    name character varying(100) NOT NULL,
    image_url text NOT NULL,
    rating double precision DEFAULT 1200,
    net_worth text DEFAULT '$0'::text NOT NULL
);


ALTER TABLE public.users OWNER TO radmin;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: radmin
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.users_id_seq OWNER TO radmin;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: radmin
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: matches id; Type: DEFAULT; Schema: public; Owner: radmin
--

ALTER TABLE ONLY public.matches ALTER COLUMN id SET DEFAULT nextval('public.matches_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: radmin
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Data for Name: matches; Type: TABLE DATA; Schema: public; Owner: radmin
--

COPY public.matches (id, winner_id, loser_id, match_time) FROM stdin;
1	1	2	2025-03-02 03:12:15.763831
2	1	2	2025-03-02 03:12:29.482378
3	1	2	2025-03-02 03:40:14.831551
4	1	2	2025-03-02 03:40:41.988954
5	7	6	2025-03-02 03:58:47.603368
6	9	3	2025-03-02 03:58:48.889305
7	9	8	2025-03-02 03:58:49.979408
8	5	2	2025-03-02 03:58:51.002887
9	8	6	2025-03-02 03:58:52.115114
10	6	2	2025-03-02 03:58:55.356994
11	9	10	2025-03-02 03:58:56.681058
12	9	5	2025-03-02 03:58:57.749643
13	3	7	2025-03-02 03:58:59.14766
14	9	5	2025-03-02 03:59:00.104746
15	9	5	2025-03-02 03:59:01.911599
16	1	9	2025-03-02 03:59:02.888838
17	1	5	2025-03-02 03:59:04.295138
18	5	9	2025-03-02 03:59:05.042751
19	5	9	2025-03-02 03:59:05.920538
20	6	8	2025-03-02 03:59:06.016526
21	6	8	2025-03-02 03:59:06.085415
22	6	8	2025-03-02 03:59:06.186202
23	6	8	2025-03-02 03:59:06.186753
24	7	8	2025-03-02 03:59:06.403833
25	6	10	2025-03-02 03:59:06.695389
26	6	2	2025-03-02 03:59:06.829765
27	6	10	2025-03-02 03:59:06.924111
28	6	10	2025-03-02 03:59:07.05791
29	10	6	2025-03-02 03:59:07.1748
30	7	4	2025-03-02 03:59:07.338006
31	7	10	2025-03-02 03:59:07.472236
32	5	2	2025-03-02 03:59:07.630411
33	5	3	2025-03-02 03:59:07.841097
34	5	3	2025-03-02 03:59:08.102238
35	5	3	2025-03-02 03:59:08.29165
36	7	3	2025-03-02 03:59:08.484733
37	6	10	2025-03-02 04:34:53.666185
38	6	10	2025-03-02 04:34:53.874173
39	9	4	2025-03-02 05:24:48.556767
40	3	6	2025-03-02 05:24:49.679489
41	2	9	2025-03-02 05:24:50.462678
42	9	3	2025-03-02 05:24:51.232064
43	8	6	2025-03-02 06:30:33.404803
44	6	8	2025-03-02 06:30:34.596981
45	1	9	2025-03-02 06:30:35.573028
46	8	6	2025-03-02 06:30:36.184577
47	1	9	2025-03-02 06:30:36.375767
48	4	8	2025-03-02 06:30:36.936587
49	1	9	2025-03-02 06:30:38.421645
50	2	8	2025-03-02 06:30:39.555387
51	9	8	2025-03-02 06:30:40.18246
52	3	4	2025-03-02 06:30:40.848222
53	4	7	2025-03-02 06:30:49.091031
54	7	3	2025-03-02 06:30:49.936102
55	10	7	2025-03-02 06:30:50.81926
56	2	4	2025-03-02 06:30:51.465715
57	4	2	2025-03-02 06:30:52.182597
58	8	4	2025-03-02 06:30:54.144398
59	3	1	2025-03-02 06:30:57.734598
60	1	3	2025-03-02 06:30:58.261215
61	5	7	2025-03-02 06:31:14.666755
62	6	5	2025-03-02 06:31:15.523323
63	1	5	2025-03-02 06:31:16.423524
64	8	10	2025-03-02 06:31:17.325717
65	6	8	2025-03-02 06:31:18.012624
66	10	6	2025-03-02 06:31:18.724928
67	10	6	2025-03-02 06:31:18.944322
68	3	7	2025-03-02 06:31:19.152507
69	1	5	2025-03-02 06:31:19.50557
70	9	1	2025-03-02 06:31:19.702643
71	1	5	2025-03-02 06:31:19.707943
72	9	2	2025-03-02 06:31:19.955493
73	4	10	2025-03-02 06:31:21.84519
74	4	3	2025-03-02 06:31:22.749769
75	1	7	2025-03-02 06:31:25.242714
76	9	6	2025-03-02 06:31:26.098282
77	9	4	2025-03-02 06:31:26.963309
78	4	5	2025-03-02 06:31:28.909525
79	5	4	2025-03-02 06:31:30.018143
80	5	4	2025-03-02 06:31:30.264096
81	10	6	2025-03-02 06:31:30.826531
82	7	10	2025-03-02 06:31:31.264037
83	7	10	2025-03-02 06:31:31.557391
84	9	10	2025-03-02 06:31:31.878577
85	10	8	2025-03-02 06:31:32.549431
86	8	10	2025-03-02 06:31:33.78732
87	3	10	2025-03-02 06:31:33.935461
88	8	10	2025-03-02 06:31:34.25974
89	3	10	2025-03-02 06:31:34.446837
90	8	10	2025-03-02 06:31:34.44751
91	3	10	2025-03-02 06:31:35.154723
92	7	4	2025-03-02 06:31:35.466531
93	3	10	2025-03-02 06:31:35.508179
94	6	4	2025-03-02 06:31:35.682223
95	6	4	2025-03-02 06:31:35.682697
96	4	2	2025-03-02 06:31:36.354339
97	4	2	2025-03-02 06:31:36.555921
98	4	2	2025-03-02 06:31:36.709585
99	4	2	2025-03-02 06:31:36.7101
100	4	2	2025-03-02 06:31:36.736073
101	6	4	2025-03-02 06:31:37.092291
102	4	3	2025-03-02 06:31:37.620028
103	8	9	2025-03-02 06:31:37.755597
104	10	1	2025-03-02 06:31:37.933013
105	8	1	2025-03-02 21:42:45.887948
106	8	1	2025-03-02 21:42:53.015106
107	10	3	2025-03-02 21:42:54.80061
108	4	10	2025-03-02 21:42:56.457201
109	3	1	2025-03-02 21:42:59.228199
110	3	8	2025-03-02 21:43:00.835299
111	9	10	2025-03-02 22:50:40.178107
112	1	5	2025-03-02 22:50:43.028608
113	8	1	2025-03-04 01:32:23.82829
114	6	9	2025-03-04 01:32:29.352461
115	8	9	2025-03-04 01:32:33.506025
116	7	5	2025-03-04 01:32:37.496074
117	1	2	2025-03-04 01:32:40.370999
118	1	4	2025-03-04 01:32:44.422124
119	7	3	2025-03-04 01:32:49.194052
120	5	9	2025-03-04 01:32:53.494315
121	5	1	2025-03-04 01:32:54.710537
122	5	3	2025-03-04 01:32:55.885481
123	3	7	2025-03-04 01:32:59.816652
124	3	7	2025-03-04 01:33:00.766762
125	9	7	2025-03-04 01:33:02.514686
126	9	7	2025-03-04 01:33:03.689476
127	2	8	2025-03-04 01:34:20.44265
128	5	8	2025-03-04 01:34:25.253185
129	10	6	2025-03-04 01:34:28.107388
130	5	3	2025-03-04 01:34:29.220251
131	5	10	2025-03-04 01:34:49.876487
132	10	1	2025-03-04 01:35:13.056524
133	5	4	2025-03-04 01:35:15.444467
134	5	4	2025-03-04 01:35:16.313252
135	10	8	2025-03-04 01:35:17.616188
136	5	4	2025-03-04 01:35:19.395997
137	8	6	2025-03-04 01:35:21.67017
138	2	8	2025-03-04 01:35:23.760029
139	7	1	2025-03-04 01:35:25.518998
140	4	8	2025-03-04 01:35:27.449641
141	4	7	2025-03-04 01:35:29.531981
142	1	6	2025-03-04 01:35:51.88434
143	2	6	2025-03-04 01:35:54.059031
144	3	6	2025-03-04 01:35:55.330779
145	5	6	2025-03-04 01:42:33.033296
146	4	1	2025-03-04 01:42:36.962229
147	4	6	2025-03-04 01:42:38.196052
148	2	1	2025-03-04 01:42:40.065944
149	6	1	2025-03-04 01:42:43.564029
150	9	8	2025-03-04 01:42:49.086324
151	4	2	2025-03-04 01:42:51.47155
152	5	2	2025-03-04 01:42:52.625613
153	8	3	2025-03-04 01:42:54.637357
154	8	6	2025-03-04 01:42:56.027338
155	5	7	2025-03-04 01:43:05.578073
156	4	5	2025-03-04 01:54:17.656198
157	4	6	2025-03-04 01:54:18.968148
158	4	8	2025-03-04 01:54:27.80564
159	8	1	2025-03-04 01:54:28.771511
160	1	2	2025-03-04 01:54:29.927454
161	6	7	2025-03-04 01:54:30.70009
162	3	1	2025-03-04 01:54:31.515533
163	9	4	2025-03-04 03:04:51.196859
164	10	7	2025-03-04 03:04:55.124156
165	5	9	2025-03-04 03:04:56.780523
166	9	7	2025-03-04 03:05:00.443869
167	9	2	2025-03-04 03:05:07.873577
168	5	4	2025-03-04 03:05:09.711328
169	4	8	2025-03-04 03:05:12.075726
170	5	8	2025-03-04 03:05:13.525925
171	9	3	2025-03-04 03:05:16.405994
172	10	2	2025-03-04 03:05:17.790628
173	24	36	2025-03-04 03:53:43.8751
174	34	50	2025-03-04 03:54:43.425666
175	24	3	2025-03-04 03:54:45.078891
176	3	2	2025-03-04 03:54:45.987402
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: radmin
--

COPY public.users (id, name, image_url, rating, net_worth) FROM stdin;
1	Elon	https://upload.wikimedia.org/wikipedia/commons/thumb/c/cb/Elon_Musk_Royal_Society_crop.jpg/360px-Elon_Musk_Royal_Society_crop.jpg	1159.8355420873647	$0
11	Elon Musk	https://commons.wikimedia.org/wiki/File:Elon_Musk_2015.jpg	1200	$359.5 billion
12	Mark Zuckerberg	https://commons.wikimedia.org/wiki/File:Mark_Zuckerberg_F8_2019_Keynote_(32830578717)_(cropped).jpg	1200	$236 billion
13	Jeff Bezos	https://commons.wikimedia.org/wiki/File:Jeff_Bezos_at_Amazon_Spheres_Grand_Opening_in_Seattle_-_2018_(39074799225).jpg	1200	$232 billion
14	Larry Ellison	https://commons.wikimedia.org/wiki/File:Larry_Ellison_on_stage.jpg	1200	$192 billion
15	Bernard Arnault	https://commons.wikimedia.org/wiki/File:Bernard_Arnault_(3).jpg	1200	$177 billion
16	Warren Buffett	https://commons.wikimedia.org/wiki/File:Warren_Buffett_KU_Visit.jpg	1200	$133 billion
17	Larry Page	https://commons.wikimedia.org/wiki/File:Larry_Page_in_the_EU_Parliament,_June_17,_2009.jpg	1200	$114 billion
7	Zuck	https://upload.wikimedia.org/wikipedia/commons/thumb/9/97/Mark_Zuckerberg_at_the_37th_G8_Summit_in_Deauville_018_square_%28cropped%29.jpg/344px-Mark_Zuckerberg_at_the_37th_G8_Summit_in_Deauville_018_square_%28cropped%29.jpg	1143.083213338833	$0
18	Sergey Brin	https://commons.wikimedia.org/wiki/File:Sergey_Brin_cropped.jpg	1200	$112 billion
19	Steve Ballmer	https://commons.wikimedia.org/wiki/File:Steve_Ballmer_at_CES_2010_cropped.jpg	1200	$109 billion
20	Francoise Bettencourt Meyers	https://commons.wikimedia.org/wiki/File:Fran%C3%A7oise_Bettencourt_Meyers.jpg	1200	$95 billion
21	Mukesh Ambani	https://commons.wikimedia.org/wiki/File:Mukesh_Ambani.jpg	1200	$93 billion
22	Carlos Slim Helu	https://commons.wikimedia.org/wiki/File:Carlos_Slim_Hel%C3%BA.jpg	1200	$90 billion
23	Michael Bloomberg	https://commons.wikimedia.org/wiki/File:Mike_Bloomberg_2019.jpg	1200	$82 billion
4	Bernard Arnault & family	https://upload.wikimedia.org/wikipedia/commons/thumb/d/de/Bernard_Arnault_%283%29_-_2017_%28cropped%29.jpg/367px-Bernard_Arnault_%283%29_-_2017_%28cropped%29.jpg	1284.2589871034493	$0
25	Zhong Shanshan	https://commons.wikimedia.org/wiki/File:Zhong_Shanshan.jpg	1200	$78 billion
5	Warren Buffet	https://upload.wikimedia.org/wikipedia/commons/thumb/d/d4/Warren_Buffett_at_the_2015_SelectUSA_Investment_Summit_%28cropped%29.jpg/360px-Warren_Buffett_at_the_2015_SelectUSA_Investment_Summit_%28cropped%29.jpg	1366.5676331578454	$0
26	Julia Koch & family	https://commons.wikimedia.org/wiki/File:Julia_Koch.jpg	1200	$75 billion
8	Jeff	https://upload.wikimedia.org/wikipedia/commons/thumb/2/2b/Jeff_Bezos%27_iconic_laugh_%28cropped%29.jpg/384px-Jeff_Bezos%27_iconic_laugh_%28cropped%29.jpg	1150.8892190315487	$0
9	Bernard Arnault & family	https://upload.wikimedia.org/wikipedia/commons/thumb/d/de/Bernard_Arnault_%283%29_-_2017_%28cropped%29.jpg/367px-Bernard_Arnault_%283%29_-_2017_%28cropped%29.jpg	1317.0066941533037	$0
27	Charles Koch	https://commons.wikimedia.org/wiki/File:Charles_Koch.jpg	1200	$75 billion
28	Jim Walton	https://commons.wikimedia.org/wiki/File:Jim_Walton.jpg	1200	$70 billion
6	Elon	https://upload.wikimedia.org/wikipedia/commons/thumb/c/cb/Elon_Musk_Royal_Society_crop.jpg/360px-Elon_Musk_Royal_Society_crop.jpg	1123.6522090228095	$0
2	Zuck	https://upload.wikimedia.org/wikipedia/commons/thumb/9/97/Mark_Zuckerberg_at_the_37th_G8_Summit_in_Deauville_018_square_%28cropped%29.jpg/344px-Mark_Zuckerberg_at_the_37th_G8_Summit_in_Deauville_018_square_%28cropped%29.jpg	1123.8164741607275	$0
10	Warren Buffet	https://upload.wikimedia.org/wikipedia/commons/thumb/d/d4/Warren_Buffett_at_the_2015_SelectUSA_Investment_Summit_%28cropped%29.jpg/360px-Warren_Buffett_at_the_2015_SelectUSA_Investment_Summit_%28cropped%29.jpg	1214.5856607297396	$0
29	Rob Walton	https://commons.wikimedia.org/wiki/File:Rob_Walton.jpg	1200	$69 billion
30	Alice Walton	https://commons.wikimedia.org/wiki/File:Alice_Walton.jpg	1200	$68 billion
31	Gautam Adani	https://commons.wikimedia.org/wiki/File:Gautam_Adani.jpg	1200	$67 billion
32	Phil Knight & family	https://commons.wikimedia.org/wiki/File:Phil_Knight.jpg	1200	$65 billion
33	Ma Huateng	https://commons.wikimedia.org/wiki/File:Ma_Huateng.jpg	1200	$64 billion
35	David Thomson & family	https://commons.wikimedia.org/wiki/File:David_Thomson.jpg	1200	$61 billion
37	Francois Pinault & family	https://commons.wikimedia.org/wiki/File:Fran%C3%A7ois_Pinault.jpg	1200	$59 billion
38	Jack Ma	https://commons.wikimedia.org/wiki/File:Jack_Ma_2015.jpg	1200	$58 billion
39	Colin Zheng Huang	https://commons.wikimedia.org/wiki/File:Colin_Zheng_Huang.jpg	1200	$57 billion
40	Dieter Schwarz	https://commons.wikimedia.org/wiki/File:Dieter_Schwarz.jpg	1200	$56 billion
41	Sheldon Adelson	https://commons.wikimedia.org/wiki/File:Sheldon_Adelson_21_June_2010.jpg	1200	$55 billion
42	Ken Griffin	https://commons.wikimedia.org/wiki/File:Ken_Griffin.jpg	1200	$54 billion
43	Gina Rinehart	https://commons.wikimedia.org/wiki/File:Gina_Rinehart.jpg	1200	$53 billion
44	Vladimir Potanin	https://commons.wikimedia.org/wiki/File:Vladimir_Potanin.jpg	1200	$52 billion
45	Tadashi Yanai & family	https://commons.wikimedia.org/wiki/File:Tadashi_Yanai.jpg	1200	$51 billion
46	Masayoshi Son	https://commons.wikimedia.org/wiki/File:Masayoshi_Son.jpg	1200	$50 billion
47	Leonid Mikhelson	https://commons.wikimedia.org/wiki/File:Leonid_Mikhelson.jpg	1200	$49 billion
48	Pony Ma	https://commons.wikimedia.org/wiki/File:Pony_Ma.jpg	1200	$48 billion
49	Alain Wertheimer	https://commons.wikimedia.org/wiki/File:Alain_Wertheimer.jpg	1200	$47 billion
51	Giovanni Ferrero	https://commons.wikimedia.org/wiki/File:Giovanni_Ferrero.jpg	1200	$46 billion
36	Michael Dell	https://commons.wikimedia.org/wiki/File:Michael_Dell.jpg	1184	$60 billion
34	MacKenzie Scott	https://commons.wikimedia.org/wiki/File:MacKenzie_Scott.jpg	1216	$62 billion
50	Gerard Wertheimer	https://commons.wikimedia.org/wiki/File:G%C3%A9rard_Wertheimer.jpg	1184	$47 billion
52	Susanne Klatten	https://commons.wikimedia.org/wiki/File:Susanne_Klatten.jpg	1200	$45 billion
53	Stefan Quandt	https://commons.wikimedia.org/wiki/File:Stefan_Quandt.jpg	1200	$44 billion
54	Li Ka-shing	https://commons.wikimedia.org/wiki/File:Li_Ka-shing.jpg	1200	$43 billion
55	He Xiangjian	https://commons.wikimedia.org/wiki/File:He_Xiangjian.jpg	1200	$42 billion
56	William Lei Ding	https://commons.wikimedia.org/wiki/File:William_Lei_Ding.jpg	1200	$41 billion
57	Gennady Timchenko	https://commons.wikimedia.org/wiki/File:Gennady_Timchenko.jpg	1200	$40 billion
24	Amancio Ortega	https://commons.wikimedia.org/wiki/File:Amancio_Ortega_Gaona.jpg	1230.2106619578592	$81 billion
3	Jeff	https://upload.wikimedia.org/wikipedia/commons/thumb/2/2b/Jeff_Bezos%27_iconic_laugh_%28cropped%29.jpg/384px-Jeff_Bezos%27_iconic_laugh_%28cropped%29.jpg	1177.664866544552	$0
58	Elon Musk	https://commons.wikimedia.org/wiki/File:Elon_Musk_2015.jpg	1200	$359.5 billion
59	Mark Zuckerberg	https://commons.wikimedia.org/wiki/File:Mark_Zuckerberg_F8_2019_Keynote_(32830578717)_(cropped).jpg	1200	$236 billion
60	Jeff Bezos	https://commons.wikimedia.org/wiki/File:Jeff_Bezos_at_Amazon_Spheres_Grand_Opening_in_Seattle_-_2018_(39074799225).jpg	1200	$232 billion
61	Larry Ellison	https://commons.wikimedia.org/wiki/File:Larry_Ellison_on_stage.jpg	1200	$192 billion
62	Bernard Arnault	https://commons.wikimedia.org/wiki/File:Bernard_Arnault_(3).jpg	1200	$177 billion
63	Warren Buffett	https://commons.wikimedia.org/wiki/File:Warren_Buffett_KU_Visit.jpg	1200	$133 billion
64	Larry Page	https://commons.wikimedia.org/wiki/File:Larry_Page_in_the_EU_Parliament,_June_17,_2009.jpg	1200	$114 billion
65	Sergey Brin	https://commons.wikimedia.org/wiki/File:Sergey_Brin_cropped.jpg	1200	$112 billion
66	Steve Ballmer	https://commons.wikimedia.org/wiki/File:Steve_Ballmer_at_CES_2010_cropped.jpg	1200	$109 billion
67	Francoise Bettencourt Meyers	https://commons.wikimedia.org/wiki/File:Fran%C3%A7oise_Bettencourt_Meyers.jpg	1200	$95 billion
68	Mukesh Ambani	https://commons.wikimedia.org/wiki/File:Mukesh_Ambani.jpg	1200	$93 billion
69	Carlos Slim Helu	https://commons.wikimedia.org/wiki/File:Carlos_Slim_Hel%C3%BA.jpg	1200	$90 billion
70	Michael Bloomberg	https://commons.wikimedia.org/wiki/File:Mike_Bloomberg_2019.jpg	1200	$82 billion
71	Amancio Ortega	https://commons.wikimedia.org/wiki/File:Amancio_Ortega_Gaona.jpg	1200	$81 billion
72	Zhong Shanshan	https://commons.wikimedia.org/wiki/File:Zhong_Shanshan.jpg	1200	$78 billion
73	Julia Koch & family	https://commons.wikimedia.org/wiki/File:Julia_Koch.jpg	1200	$75 billion
74	Charles Koch	https://commons.wikimedia.org/wiki/File:Charles_Koch.jpg	1200	$75 billion
75	Jim Walton	https://commons.wikimedia.org/wiki/File:Jim_Walton.jpg	1200	$70 billion
76	Rob Walton	https://commons.wikimedia.org/wiki/File:Rob_Walton.jpg	1200	$69 billion
77	Alice Walton	https://commons.wikimedia.org/wiki/File:Alice_Walton.jpg	1200	$68 billion
78	Gautam Adani	https://commons.wikimedia.org/wiki/File:Gautam_Adani.jpg	1200	$67 billion
79	Phil Knight & family	https://commons.wikimedia.org/wiki/File:Phil_Knight.jpg	1200	$65 billion
80	Ma Huateng	https://commons.wikimedia.org/wiki/File:Ma_Huateng.jpg	1200	$64 billion
81	MacKenzie Scott	https://commons.wikimedia.org/wiki/File:MacKenzie_Scott.jpg	1200	$62 billion
82	David Thomson & family	https://commons.wikimedia.org/wiki/File:David_Thomson.jpg	1200	$61 billion
83	Michael Dell	https://commons.wikimedia.org/wiki/File:Michael_Dell.jpg	1200	$60 billion
84	Francois Pinault & family	https://commons.wikimedia.org/wiki/File:Fran%C3%A7ois_Pinault.jpg	1200	$59 billion
85	Jack Ma	https://commons.wikimedia.org/wiki/File:Jack_Ma_2015.jpg	1200	$58 billion
86	Colin Zheng Huang	https://commons.wikimedia.org/wiki/File:Colin_Zheng_Huang.jpg	1200	$57 billion
87	Dieter Schwarz	https://commons.wikimedia.org/wiki/File:Dieter_Schwarz.jpg	1200	$56 billion
88	Sheldon Adelson	https://commons.wikimedia.org/wiki/File:Sheldon_Adelson_21_June_2010.jpg	1200	$55 billion
89	Ken Griffin	https://commons.wikimedia.org/wiki/File:Ken_Griffin.jpg	1200	$54 billion
90	Gina Rinehart	https://commons.wikimedia.org/wiki/File:Gina_Rinehart.jpg	1200	$53 billion
91	Vladimir Potanin	https://commons.wikimedia.org/wiki/File:Vladimir_Potanin.jpg	1200	$52 billion
92	Tadashi Yanai & family	https://commons.wikimedia.org/wiki/File:Tadashi_Yanai.jpg	1200	$51 billion
93	Masayoshi Son	https://commons.wikimedia.org/wiki/File:Masayoshi_Son.jpg	1200	$50 billion
94	Leonid Mikhelson	https://commons.wikimedia.org/wiki/File:Leonid_Mikhelson.jpg	1200	$49 billion
95	Pony Ma	https://commons.wikimedia.org/wiki/File:Pony_Ma.jpg	1200	$48 billion
96	Alain Wertheimer	https://commons.wikimedia.org/wiki/File:Alain_Wertheimer.jpg	1200	$47 billion
97	Gerard Wertheimer	https://commons.wikimedia.org/wiki/File:G%C3%A9rard_Wertheimer.jpg	1200	$47 billion
98	Giovanni Ferrero	https://commons.wikimedia.org/wiki/File:Giovanni_Ferrero.jpg	1200	$46 billion
99	Susanne Klatten	https://commons.wikimedia.org/wiki/File:Susanne_Klatten.jpg	1200	$45 billion
100	Stefan Quandt	https://commons.wikimedia.org/wiki/File:Stefan_Quandt.jpg	1200	$44 billion
101	Li Ka-shing	https://commons.wikimedia.org/wiki/File:Li_Ka-shing.jpg	1200	$43 billion
102	He Xiangjian	https://commons.wikimedia.org/wiki/File:He_Xiangjian.jpg	1200	$42 billion
103	William Lei Ding	https://commons.wikimedia.org/wiki/File:William_Lei_Ding.jpg	1200	$41 billion
104	Gennady Timchenko	https://commons.wikimedia.org/wiki/File:Gennady_Timchenko.jpg	1200	$40 billion
\.


--
-- Name: matches_id_seq; Type: SEQUENCE SET; Schema: public; Owner: radmin
--

SELECT pg_catalog.setval('public.matches_id_seq', 176, true);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: radmin
--

SELECT pg_catalog.setval('public.users_id_seq', 104, true);


--
-- Name: matches matches_pkey; Type: CONSTRAINT; Schema: public; Owner: radmin
--

ALTER TABLE ONLY public.matches
    ADD CONSTRAINT matches_pkey PRIMARY KEY (id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: radmin
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: matches matches_loser_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: radmin
--

ALTER TABLE ONLY public.matches
    ADD CONSTRAINT matches_loser_id_fkey FOREIGN KEY (loser_id) REFERENCES public.users(id);


--
-- Name: matches matches_winner_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: radmin
--

ALTER TABLE ONLY public.matches
    ADD CONSTRAINT matches_winner_id_fkey FOREIGN KEY (winner_id) REFERENCES public.users(id);


--
-- PostgreSQL database dump complete
--

