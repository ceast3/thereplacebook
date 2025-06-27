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
    winner_id integer NOT NULL,
    loser_id integer NOT NULL
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
    name text NOT NULL,
    image_url text NOT NULL,
    rating double precision DEFAULT 1200,
    net_worth text DEFAULT '$0'::text NOT NULL,
    biography text,
    birthdate date,
    nationality text,
    industry text,
    company text,
    source_of_wealth text,
    philanthropy text,
    notable_achievements text,
    website text,
    twitter_handle text,
    linkedin_profile text,
    quote text,
    parental_wealth text
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

COPY public.matches (id, winner_id, loser_id) FROM stdin;
1	1	2
2	3	2
3	4	10
4	10	3
5	2	3
6	7	6
7	6	2
8	6	7
9	7	9
10	10	2
11	9	3
12	4	3
13	8	5
14	5	7
15	7	2
16	9	6
17	9	5
18	6	10
19	10	6
20	6	8
21	5	1
22	5	1
23	1	5
24	1	5
25	8	5
26	8	4
27	10	1
28	9	10
29	7	10
30	7	1
31	6	8
32	6	10
33	2	10
34	7	8
35	10	2
36	7	9
37	2	7
38	2	3
39	3	7
40	1	10
41	10	8
42	2	6
43	1	6
44	1	10
45	4	6
46	5	7
47	6	7
48	7	10
49	5	7
50	7	8
51	1	7
52	6	9
53	6	4
54	6	5
55	4	8
56	6	8
57	2	3
58	2	5
59	6	8
60	1	9
61	9	4
62	2	6
63	2	6
64	5	10
65	1	2
66	1	10
67	4	7
68	7	6
69	5	8
70	2	1
71	9	3
72	4	9
73	10	3
74	9	3
75	10	8
76	2	1
77	2	3
78	3	5
79	6	10
80	1	5
81	1	9
82	3	9
83	3	8
84	7	8
85	2	4
86	10	9
87	2	7
88	1	5
89	9	10
90	7	9
91	4	9
92	10	5
93	2	10
94	4	10
95	1	4
96	8	6
97	10	3
98	8	2
99	8	5
100	7	10
101	5	6
102	10	8
103	6	8
104	5	7
105	9	4
106	10	4
107	2	6
108	7	9
109	4	2
110	7	1
111	5	7
112	5	8
113	5	1
114	9	6
115	2	10
116	2	4
117	8	1
118	4	8
119	1	9
120	1	7
121	2	5
122	6	2
123	2	5
124	1	8
125	6	4
126	1	10
127	10	7
128	9	10
129	5	9
130	5	4
131	9	8
132	2	3
133	1	7
134	3	8
135	1	3
136	3	6
137	10	6
138	3	7
139	4	8
140	1	13
141	13	15
142	2	16
143	1	16
144	11	12
145	1	8
146	2	12
147	16	4
148	1	13
149	4	8
150	3	14
151	12	15
152	13	4
153	11	22
154	3	27
155	24	28
156	11	2
157	11	6
158	24	7
159	1	25
160	10	4
161	14	24
162	20	11
163	25	14
164	6	14
165	13	8
166	4	14
167	25	15
168	28	9
169	5	4
170	28	24
171	26	6
172	22	9
173	28	9
174	2	13
175	13	15
176	24	26
177	6	25
178	15	8
179	9	3
180	1	10
181	2	14
182	3	14
183	9	8
184	26	9
185	12	5
186	5	7
187	15	33
188	22	7
189	27	3
190	20	24
191	6	1
192	11	8
193	20	28
194	27	22
195	4	14
196	4	5
197	5	24
198	28	27
199	11	5
200	5	6
201	10	12
202	14	12
203	1	2
204	3	11
205	28	27
206	9	15
207	33	1
208	7	2
209	12	33
210	24	4
211	5	14
212	25	2
213	15	11
214	20	10
215	6	10
216	2	4
217	2	24
218	24	33
219	22	4
220	24	25
221	5	8
222	14	33
223	7	5
224	5	24
225	24	20
226	28	25
227	5	14
228	7	25
229	2	10
230	28	12
231	3	4
232	28	6
233	25	15
234	6	8
235	1	13
236	12	6
237	28	10
238	2	12
239	26	11
240	14	9
241	7	28
242	11	10
243	9	24
244	8	15
245	11	6
246	2	27
247	28	4
248	5	22
249	20	22
250	11	5
251	6	25
252	14	9
253	3	27
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: radmin
--

COPY public.users (id, name, image_url, rating, net_worth, biography, birthdate, nationality, industry, company, source_of_wealth, philanthropy, notable_achievements, website, twitter_handle, linkedin_profile, quote, parental_wealth) FROM stdin;
8	Warren Buffett	https://upload.wikimedia.org/wikipedia/commons/d/d4/Warren_Buffett_at_the_2015_SelectUSA_Investment_Summit_%28cropped%29.jpg	1017.257886788462	$156.0 billion	Warren Buffett is an American investor, business magnate, and philanthropist. \n                 He is the chairman and CEO of Berkshire Hathaway and is considered one of the most successful investors of all time.	1930-08-30	American	Finance, Investments	Berkshire Hathaway	Self-Made; Built fortune through value investing and strategic acquisitions	Committed to donating over 99% of his wealth, major contributions to the Gates Foundation	Built Berkshire Hathaway into a multinational conglomerate, known as the "Oracle of Omaha"	https://www.berkshirehathaway.com	\N	\N	\N	His father, Howard Buffett, was a U.S. congressman and stockbroker, giving him early exposure to finance.
7	Larry Page	https://upload.wikimedia.org/wikipedia/commons/2/26/Larry_Page_in_the_European_Parliament%2C_17.06.2009.jpg	1202.2730128390251	$161.8 billion	Larry Page is an American computer scientist and entrepreneur best known as the co-founder of Google. \n                 He played a key role in developing the PageRank algorithm, which revolutionized web search engines.	1973-03-26	American	Technology, Internet	Google (Alphabet)	Partially Self-Made; Early wealth from family, later built fortune through Google	Focused on renewable energy, health sciences, and AI research	Co-founded Google, developed PageRank, and led Alphabet Inc.	https://www.google.com	\N	\N	\N	His father, Carl Victor Page Sr., was a pioneering computer scientist and professor, giving him early access to technology.
10	Steve Ballmer	https://upload.wikimedia.org/wikipedia/commons/b/b6/Steve_ballmer_2007_outdoors2.jpg	1136.0117774143405	$141.1 billion	Steve Ballmer is an American businessman and investor who served as the CEO of Microsoft from 2000 to 2014. \n                 Under his leadership, Microsoft expanded into enterprise software, cloud computing, and hardware.	1956-03-24	American	Technology, Sports Ownership	Microsoft, Los Angeles Clippers	Self-Made; Made fortune through Microsoft leadership and stock holdings	Donated billions to education, public health, and civic engagement initiatives	Led Microsoft’s expansion, acquired Skype, and became a major sports team owner	https://www.microsoft.com	\N	\N	\N	His father, Frederic Ballmer, was a Ford Motor Company manager, providing a stable middle-class upbringing.
1	Elon Musk	https://upload.wikimedia.org/wikipedia/commons/c/cb/Elon_Musk_Royal_Society_crop.jpg	1343.6983271910872	$380.1 billion	Elon Musk is a business magnate, investor, and CEO of Tesla, SpaceX, and Neuralink. \n                 Born in South Africa, he later moved to the U.S., co-founding PayPal and investing in Tesla and SpaceX.	1971-06-28	South African, American, Canadian	Technology, Space Exploration, Automotive	Tesla, SpaceX, Neuralink, The Boring Company	Partially Self-Made; Early wealth from family, later built fortunes through investments and startups	Donated towards AI research, space exploration, and renewable energy initiatives	Helped commercialize electric vehicles, pioneered private space travel, and advanced AI research	https://www.tesla.com	@elonmusk	https://www.linkedin.com/in/elonmusk/	\N	His father, Errol Musk, was a wealthy engineer who owned shares in an emerald mine in Zambia.
5	Bernard Arnault	https://upload.wikimedia.org/wikipedia/commons/a/a5/Bernard_Arnault_%282%29_-_2017_%28cropped%29.jpg	1254.7341565690901	$193.0 billion	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N	\N
11	Mukesh Ambani	https://upload.wikimedia.org/wikipedia/commons/6/69/Mukesh_Ambani.jpg	1257.0132635016023	$94.3 billion	Mukesh Ambani is an Indian billionaire businessman and chairman of Reliance Industries, one of India’s largest conglomerates.	1957-04-19	Indian	Diversified Industries	Reliance Industries	Inherited & Expanded Wealth; Took over Reliance from his father and scaled it into a global powerhouse	Invests in education, healthcare, and rural development through the Reliance Foundation	Expanded Reliance Industries into telecom (Jio), retail, and energy sectors	https://www.ril.com	@mukeshambani	\N	\N	Son of Dhirubhai Ambani, founder of Reliance Industries
14	Amancio Ortega	https://i.namu.wiki/i/gbQG8PAeUoS-r7m9HDdOp7uPlq5zkfjWSHjv0tXb9HrnRGjC_n7Xo7RtW-AcatgpBey0EVSxyCavs8tajX-oZ6s0jMbkR9SGhL4ZUS4u0rqDFiAkZHekJYOIqdxvKxX0mMFcCqn2MDYM-wE5_oJ5UqAIvZVBXybOMCdiYsd5oSA.webp	1164.484637035874	$81.4 billion	Amancio Ortega is a Spanish billionaire businessman, founder of Inditex, the fashion group behind Zara.	1936-03-28	Spanish	Fashion, Retail	Inditex (Zara)	Self-Made; Built Zara and Inditex into the world’s largest fashion retailer	Donates to cancer research and education through his charitable foundation	Transformed the fashion industry with fast fashion and supply chain innovations	https://www.inditex.com	\N	\N	\N	Born to a railway worker, started working in textiles at a young age and built his empire from the ground up
3	Jeff Bezos	https://upload.wikimedia.org/wikipedia/commons/3/33/Jeff_Bezos_2016.jpg	1245.0954866634431	$233.5 billion	Jeff Bezos is an American entrepreneur and investor best known as the founder of Amazon. \n                 He revolutionized e-commerce, cloud computing, and logistics, making Amazon one of the most valuable companies in the world.	1964-01-12	American	Technology, E-Commerce	Amazon	Partially Self-Made; Early financial support from family, later built fortune through Amazon	Founded the Bezos Earth Fund, donated to education, space exploration, and climate change initiatives	Created Amazon, expanded into cloud computing (AWS), and commercialized space travel via Blue Origin	https://www.amazon.com	@JeffBezos	\N	\N	His adoptive father, Miguel Bezos, was an Exxon engineer, and his maternal grandparents owned a large ranch in Texas, giving him early financial security.
15	Francoise Bettencourt Meyers	https://cdn.sanity.io/images/vxy259ii/production/42137f97f4d4f789394d869c2ff2affe8f4df268-960x631.jpg?auto=format&crop=entropy&fit=crop&h=631&q=80&w=808	1133.4782708752384	$95.1 billion	Francoise Bettencourt Meyers is a French businesswoman and philanthropist, heiress to the L’Oreal fortune.	1953-07-10	French	Cosmetics	L’Oreal	Inherited Wealth; Heiress to the L’Oreal empire	Committed to philanthropy through the Bettencourt Schueller Foundation	Oversees L’Oreal’s expansion while engaging in philanthropic efforts in science and the arts	https://www.loreal.com	\N	\N	\N	Daughter of Liliane Bettencourt, former L’Oreal owner and one of the world’s richest women
12	Carlos Slim Helu	https://upload.wikimedia.org/wikipedia/commons/d/df/Carlos_Slim_Helú.jpg	1184.6995393226175	$90.2 billion	Carlos Slim Helu is a Mexican business magnate, investor, and philanthropist known for his extensive holdings in telecommunications.	1940-01-28	Mexican	Telecommunications	Grupo Carso, America Movil	Self-Made; Built his fortune through aggressive investments in telecommunications and infrastructure	Founder of the Carlos Slim Foundation, supporting education, health, and sports	Expanded America Movil into Latin America’s largest telecom provider	https://www.carso.com.mx	\N	\N	\N	His father was a successful businessman in Mexico who instilled strong investment principles in him
20	Jim Walton	https://upload.wikimedia.org/wikipedia/commons/1/16/Jim_Walton_%28cropped%29.jpg	1259.0004014935173	$69.7 billion	Jim Walton is an American businessman and heir to the Walmart fortune, serving as chairman of Arvest Bank.	1948-06-07	American	Retail, Banking	Walmart, Arvest Bank	Inherited Wealth; Heiress to the Walmart fortune	Major donor to education and community development projects	Heads Arvest Bank, expanding Walmart’s financial influence	https://www.walmart.com	\N	\N	\N	Son of Walmart founder Sam Walton
22	Alice Walton	https://upload.wikimedia.org/wikipedia/commons/a/af/Alice_Walton_portrait_%28cropped%29.jpg	1182.5705708439527	$68.2 billion	Alice Walton is an American heiress and philanthropist, known for her contributions to the arts and founding the Crystal Bridges Museum.	1949-10-07	American	Retail	Walmart	Inherited Wealth; Heiress to the Walmart fortune	Major donor to the arts, founder of the Crystal Bridges Museum of American Art	Plays a significant role in arts philanthropy while maintaining Walmart shares	https://www.walmart.com	\N	\N	\N	Daughter of Walmart founder Sam Walton
13	Michael Bloomberg	https://upload.wikimedia.org/wikipedia/commons/1/10/Mike_Bloomberg_Headshot_%283x4_cropped%29.jpg	1210.965633886289	$82.1 billion	Michael Bloomberg is an American businessman, philanthropist, and politician, best known as the founder of Bloomberg LP.	1942-02-14	American	Finance, Media	Bloomberg LP	Self-Made; Built Bloomberg LP after being fired from Salomon Brothers	Donated billions through Bloomberg Philanthropies, focusing on climate change, public health, and education	Expanded Bloomberg LP into a financial data powerhouse, served as Mayor of New York City	https://www.bloomberg.com	@MikeBloomberg	https://www.linkedin.com/in/mikebloomberg/	\N	Born into a middle-class family with no significant inherited wealth
24	Gautam Adani	https://upload.wikimedia.org/wikipedia/commons/f/ff/Gautam_Adani.jpg	1202.9307230416575	$67.2 billion	Gautam Adani is an Indian billionaire industrialist, founder and chairman of the Adani Group, a multinational conglomerate in India.	1962-06-24	Indian	Infrastructure, Energy	Adani Group	Self-Made; Built Adani Group into one of India’s largest conglomerates	Contributes to healthcare, education, and disaster relief through the Adani Foundation	Expanded Adani Group into ports, power, and green energy sectors	https://www.adani.com	@gautam_adani	\N	\N	Born to a textile trader, started in diamond trading before founding Adani Enterprises
4	Larry Ellison	https://upload.wikimedia.org/wikipedia/commons/0/00/Larry_Ellison_picture.png	1123.960473663535	$193.0 billion	Larry Ellison is an American business magnate, best known as the co-founder of Oracle Corporation. \n                 He played a crucial role in developing database software that transformed enterprise computing.	1944-08-17	American	Technology, Enterprise Software	Oracle Corporation	Self-Made; Built Oracle from the ground up with co-founders	Donated billions towards medical research, education, and climate initiatives	Co-founded Oracle, revolutionized database software, and expanded into cloud computing	https://www.oracle.com	\N	\N	\N	Born to an unwed Jewish mother and raised by his aunt and uncle in Chicago; no significant inherited wealth.
33	Jensen Huang	https://upload.wikimedia.org/wikipedia/commons/3/36/Jensen_Huang_20231109_%28cropped2%29.jpg	1153.425842874712	$57.3 billion	Jensen Huang is a Taiwanese-American billionaire businessman, co-founder, and CEO of NVIDIA, a leader in graphics processing and AI computing.	1963-02-17	Taiwanese-American	Technology, Semiconductors	NVIDIA	Self-Made; Built NVIDIA into a global leader in AI and gaming hardware	Supports AI research, education, and philanthropic initiatives in science and technology	Revolutionized the gaming industry with GPUs, expanded into AI computing and autonomous systems	https://www.nvidia.com	@nvidia	https://www.linkedin.com/in/jensen-huang/	\N	Born in Taiwan, immigrated to the U.S. and studied electrical engineering, working his way into the tech industry
6	Bill Gates	https://upload.wikimedia.org/wikipedia/commons/c/cc/Bill_Gates%2C_September_2024.jpg	1189.213223250648	$166.7 billion	Bill Gates is an American business magnate, philanthropist, and co-founder of Microsoft. \n                 His work in software development helped shape the modern personal computing industry.	1955-10-28	American	Technology, Software	Microsoft	Partially Self-Made; Early wealth from family, later built Microsoft into a global powerhouse	Co-founded the Bill & Melinda Gates Foundation, focusing on global health, education, and poverty alleviation	Created Microsoft, revolutionized the software industry, and became one of the world’s biggest philanthropists	https://www.microsoft.com	@BillGates	https://www.linkedin.com/in/williamhgates	\N	His father, William H. Gates Sr., was a prominent lawyer, and his mother, Mary Gates, was a business executive and bank board member, providing financial security and business connections.
2	Mark Zuckerberg	https://upload.wikimedia.org/wikipedia/commons/7/71/Mark_Zuckerberg_2019_%28cropped%29.jpg	1346.1333077942138	$235.6 billion	Mark Zuckerberg is an American entrepreneur and software engineer, best known as the co-founder and CEO of Meta (formerly Facebook). \n                 He launched Facebook in 2004 from his Harvard dorm room, which became the world’s largest social media platform.	1984-05-14	American	Technology, Social Media	Meta (Facebook)	Partially Self-Made; Early wealth from family, later built fortune through Facebook	Co-founded the Chan Zuckerberg Initiative, pledged to donate 99% of his Facebook shares	Created Facebook, transformed digital social networking, and expanded into AI and the Metaverse	https://www.meta.com	\N	\N	\N	His father, Edward Zuckerberg, was a successful dentist, and his mother, Karen, was a psychiatrist, giving him a comfortable upper-middle-class upbringing.
25	Phil Knight	https://upload.wikimedia.org/wikipedia/commons/b/bc/Philknightfootball.jpg	1174.0142108602952	$65.4 billion	Phil Knight is an American billionaire business magnate, co-founder of Nike, and one of the most influential figures in sports apparel.	1938-02-24	American	Sportswear	Nike	Self-Made; Built Nike into a global sportswear powerhouse	Donated billions to education and athletics programs	Transformed the sports apparel industry with innovative marketing and branding	https://www.nike.com	\N	\N	\N	Born into a middle-class family, father was a newspaper publisher
28	Rob Walton	https://upload.wikimedia.org/wikipedia/commons/9/9d/S._Robson_Walton.jpg	1283.4511655308177	$68.1 billion	Rob Walton is an American billionaire businessman and heir to the Walmart fortune, having served as chairman of the company from 1992 to 2015.	1944-10-27	American	Retail	Walmart	Inherited Wealth; Heiress to the Walmart fortune	Supports environmental and educational initiatives through family foundations	Played a key role in Walmart’s expansion and governance	https://www.walmart.com	\N	\N	\N	Son of Walmart founder Sam Walton
9	Sergey Brin	https://upload.wikimedia.org/wikipedia/commons/d/d3/Sergey_Brin_Ted_2010_%28cropped%29.jpg	1146.0376628686274	$152.1 billion	Sergey Brin is a Russian-American computer scientist and internet entrepreneur, best known as the co-founder of Google. \n                 Alongside Larry Page, he developed Google’s search engine and played a critical role in shaping modern web technology.	1973-08-21	American	Technology, Internet	Google (Alphabet)	Self-Made; Built Google from the ground up with Larry Page	Donated towards medical research, Parkinson’s disease initiatives, and climate change projects	Co-founded Google, helped develop search algorithms, and led projects like Google X and AI advancements	https://www.google.com	\N	\N	\N	His father, Michael Brin, was a respected mathematician and professor, and his mother, Eugenia Brin, was a NASA researcher, providing strong academic influences.
26	Michael Dell	https://upload.wikimedia.org/wikipedia/commons/c/ce/Michael_Dell_2010.jpg	1229.1283754283002	$60.3 billion	Michael Dell is an American billionaire businessman and philanthropist, best known as the founder and CEO of Dell Technologies.	1965-02-23	American	Technology	Dell Technologies	Self-Made; Built Dell into a leading PC and enterprise technology company	Supports global education, healthcare, and entrepreneurship through the Michael & Susan Dell Foundation	Revolutionized the PC industry with direct-to-consumer sales	https://www.dell.com	@MichaelDell	https://www.linkedin.com/in/michaeldell/	\N	Father was an orthodontist, mother was a stockbroker, providing a financially comfortable upbringing
27	David Thomson & Family	https://i.namu.wiki/i/gbQG8PAeUoS-r7m9HDdOp8AANplZ828PgnTHbcKLmA6Ni4tw0qa2TXffHxMncDV2uZ8e_znxLPA2LrphZHr8e68VwXCw7SJqpzFzXuVXwQHGeut8LdQGk2f6dq1HqEuVT5aNBw1rEbVtAjTw7r5IQvIsnYQiorUbWUJZ2JaMuWM.webp	1164.2849079547852	$59.7 billion	David Thomson is a Canadian media magnate, chairman of Thomson Reuters, and a key player in global information services.	1957-06-12	Canadian	Media	Thomson Reuters	Inherited Wealth; Inherited media empire from father Kenneth Thomson	Supports arts, culture, and media initiatives through the Thomson family foundation	Oversees expansion of Thomson Reuters into global digital media	https://www.thomsonreuters.com	\N	\N	\N	Son of Kenneth Thomson, former chairman of Thomson Corporation
\.


--
-- Name: matches_id_seq; Type: SEQUENCE SET; Schema: public; Owner: radmin
--

SELECT pg_catalog.setval('public.matches_id_seq', 253, true);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: radmin
--

SELECT pg_catalog.setval('public.users_id_seq', 33, true);


--
-- Name: matches matches_pkey; Type: CONSTRAINT; Schema: public; Owner: radmin
--

ALTER TABLE ONLY public.matches
    ADD CONSTRAINT matches_pkey PRIMARY KEY (id);


--
-- Name: users unique_name; Type: CONSTRAINT; Schema: public; Owner: radmin
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT unique_name UNIQUE (name);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: radmin
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users allow_all; Type: POLICY; Schema: public; Owner: radmin
--

CREATE POLICY allow_all ON public.users FOR SELECT USING (true);


--
-- Name: users; Type: ROW SECURITY; Schema: public; Owner: radmin
--

ALTER TABLE public.users ENABLE ROW LEVEL SECURITY;

--
-- PostgreSQL database dump complete
--

