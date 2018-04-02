{-
bot = bot
isone = fst (0 + 1, bot)

isone
1
isbot (0 + 1, bot)

isbot = snd (0 + 1, bot)
isbot

seq = let seqn = \n -> n : seqn (n + 1) in seqn 0

seqnm n m = take m (map (+n) seq)

seq1 n = drop n seq

conseq = zip seq (seq1 1)

nsum n = sum (take n+1 seq)

rightTriangles = [ (a,b,c) | c <- [1..1000], b <- [1..c], a <- [1..b], a^2 + b^2 == c^2, a+b+c <= 24]

rightTriangles2 = take 2 rightTriangles

data Abb a = Hoja | Node (Abb a) a (Abb a) deriving (Show,Read,Eq)

mapprima::(a->b)->(Abb a)->(Abb b)
mapprima f Hoja = Hoja
mapprima f (Node l a r) = Node (mapprima f l) (f a) (mapprima f r)


foldprima f n Hoja = n
foldprima f n (Node l a r) = (foldprima f (foldprima f (f n a) l) r)

mayor0 arbol = foldprima (&&) True (mapprima (>0) arbol)

fn = ceiling . negate . tan . cos . max 100
-}

-- Type Classes y Functores

-- 1

data MyMaybe a = Nada | Solo a deriving Show

instance Functor MyMaybe where
    fmap _ Nada = Nada
    fmap f (Solo a) = Solo (f a)

data YaSea e a = Izq e | Der a

instance Functor (YaSea e) where
  fmap _ (Izq e) = Izq e
  fmap f (Der x) = Der (f x)

newtype Prod e a = Prod(e,a)

instance Functor (Prod e) where
    fmap f (Prod(e,a)) = Prod(e, f a) 

-- 2

-- fmap id = id

-- fmap id (Prod(e,a)) = 
--     {def de fmap}
-- Prod(e, id a) =
--     {def de funcion id}
-- Prod(e, a)

-- fmap (g.f) = fmap g . fmap f

-- fmap (g . f) Prod(e,a) = 
--     {def de fmap}
-- Prod(e, (g.f) a) =
--     {def de .}
-- Prod(e, g(f(a))) =
--     {def de fmap}
-- fmap g $ Prod (e, f a) =
--     {def de fmap}
-- fmap g $ fmap f $ Prod (e,a)

-- 4 

newtype Any = Any {any' :: Bool}

instance Monoid Any where
    mempty = Any False
    mappend a b = Any $ any' a || any' b

-- 5

newtype All = All {all' :: Bool}

instance Monoid All where
    mempty = All True
    mappend a b = All $ all' a && all' b

newtype IntSum = IntSum {int' :: Int}

instance Monoid IntSum where
    mempty = IntSum 0
    mappend a b = IntSum $ int' a + int' b

newtype IntProd = IntProd {intp :: Int}

instance Monoid IntProd where
    mempty = IntProd 1
    mappend a b = IntProd $ intp a * intp b

-- 6

instance Monoid e => Applicative (Prod e) where
    pure a = (Prod(mempty,a))
    (Prod (e,f)) <*> (Prod (e',a)) = Prod (mappend e e',f a)



