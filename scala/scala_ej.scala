def last[A] (B : List[A]) : Option[A] = {
 	B match {
    case Nil => None
    case h :: Nil => Some(h)
    case h :: hs => last(hs)
  }
}


def last_two[A] (B : List[A]) : Option[(A,A)] = {
 	B match {
    case Nil => None
    case h :: Nil => None
    case h :: a :: Nil => Some(h,a)
    case h :: hs => last_two(hs)
  }
}

def fold_left[A,B] (f : A => B => A, a : A, b : List[B]) : A = {
  b match {
    case Nil => a
    case h::hs => fold_left(f,f(a)(h),hs)
  }
}

def sinceros (f : Int => Boolean, a : List[Int]) : List[Boolean] = {
  return (a.filter( (x) => x != 0) map f);
}

def compress[A] (a : List[A]) : List[A] = {
  a match  {
    case Nil => Nil
    case b :: Nil => b :: Nil
    case b :: hs => hs match {
//                      case Nil => b :: Nil;
                      case x::xs =>
                          if (b == x) {
                            compress(hs);
                          } else {
                            b :: compress(hs);
                          }
                        }
  }
}

case class Materia(nombre : String, nota : Float);
case class Estudiante(libreta : Int, nombre : String, materias : List[Materia]);


sealed abstract class Eitherp[+A, +B] extends AnyRef;

final case class Leftp[+A, +B](a: A) extends Eitherp[A, B] with Product with Serializable;
final case class Rightp[+A, +B](b: B) extends Eitherp[A, B] with Product with Serializable;
