//wielo-wątkowość
//use std::{thread, time};

//zabezpieczenia podczas wielowątkowości
//use std::mutex;


#[derive(Clone)]
struct Cube {
    p: f64, 
    s: f64,
    v: f64, 
    m: f64
}

impl Cube {
    fn new(pos: f64, size: f64, vel: f64, mass: f64) -> Cube {
        Cube {
            p: pos,
            s: size,
            v: vel,
            m: mass,
        }
    }
}   


//większa liczba -- większa precyzja
//jeżeli symulacja jest zbyt powolna, 
//można zmniejszyć tą liczbę, jednak
//nie może być zbyt niska.
//Przy PI_D=5, minimum dla tej liczby to 100.
//zalecana wartość na moim komputerze to 10_000
const TIMESTEP: usize = 10_000;


//ta wartość wskazuje ilości cyfr PI po przecinku
//wartości powyżej 5 zajmują długo do obliczenia
const PI_D: usize = 5;



//limit od prawej strony
const SIZE: usize = 100;
//wizualna wielkość drugiego elementu
const MAX_Y: usize = 20;

fn main() {
    
    //obie sumy są dodawane do siebie, tworząc 'n', a liczba cyfr pi to 'd'
    //wzór to: d = n + 1
    //obliczenia szybko zaczynają zabierać coraz więcej czasu
    //
    //ten podział istnieje przez ograniczenia sprzętowe, te liczby to 10^n więc
    //szybko stają się za duże dla komputera, przez rozdzielenie je na 2 osobne liczby,
   
    //dlatego też liczbę PI_D dzielę na 2 mniejsze
    const PI_D_1: usize = PI_D/2;
    
    const PI_D_2: usize = {
        if PI_D % 2 == 0 {
            PI_D/2
        } else {
            PI_D/2+1
        }
    };

    let mu: u64 = {
        let mut a: u64 = 1;
        for _ in 0..PI_D_1 {
            a *= 100;
        }
        a
    }; 
    let mf: f64 = {
        let mut a: f64 = 1.0;
        for _ in 0..PI_D_2 {
            a = a/100.0;
        }
        a
    };

    //                       pos size vel mass
    let mut box1 = Cube::new(5.0, 5.0, 0.0, mf);
    let mut box2 = Cube::new(40.0, MAX_Y as f64, -1.0, mu as f64);
    let mut cnt: u32 = 0;
    
    //nieużywana zmienna, która ma szansę później być użyta
    //let frame = time::Duration::from_millis(1);

    // formułka 1d na prędkość bez strat energii:
    // 
    // https://en.wikipedia.org/wiki/Elastic_collision#Equations
    // 
    // v prędkość po
    //  u prędkość przed
    //  1, 2 numery klocków
    //  v1 = ((m1-m2)/(m1+m2))u1 + ((2*m2)/(m1+m2))*u2
    //  v2 = ((2*m1)/(m1+m2))u1 + ((m2-m1)/(m1+m2))*u2

    //  a teraz to tylko że na kod
    loop {

        //przesuwamy objekt o prędkość
        box1.p += box1.v / TIMESTEP as f64;
        box2.p += box2.v / TIMESTEP as f64;
        
        //sprawdzamy czy box2 nie uciekł
        if box2.p as usize > SIZE { //off grid
            println!();
            println!("Finalna liczba kolizji: {}", cnt);
            std::process::exit(0);
        }

        //sprawdzamy czy box1 nie dotknął już ściany
        if box1.p <= 0.0 {
            //zakładamy, że ściana ma nieskończoną masę, więc bezstratnie
            //odbija objekt czyli jego kierunek się odwraca
            box1.v = box1.v * -1.0;
            cnt += 1;
        }

        //detekcja kolizji, sprawdzamy tylko box1,
        //ponieważ on będzię miał styczność z zarówno box2 i ścianą.
        //jeżeli tak, to aplikujemy wzór
        
        //jeżeli prawa ściana box1 dotyka lub przekracza lewą box2 to:
        if (box1.p + box1.s) >= box2.p {
            

            //tmp ponieważ zmiany zaaplikujemy na końcu
            //  v1 = ((m1-m2)/(m1+m2))u1 
            //     + ((2*m2)/(m1+m2))*u2
            
            //  v2 = ((2*m1)/(m1+m2))u1 
            //     + ((m2-m1)/(m1+m2))*u2
            
            let tmp1 = (((box1.m - box2.m) / (box1.m + box2.m)) * box1.v)
                        + (((2.0 * box2.m) / (box1.m + box2.m)) * box2.v);
            let tmp2 = (((2.0 * box1.m) / (box1.m + box2.m)) * box1.v)
                        + (((box2.m - box1.m) / (box1.m + box2.m)) * box2.v);

            box1.v = tmp1;
            box2.v = tmp2;
            cnt += 1;
        }  

        //kolejny licznik
        let mut used: usize = 0;

        //czyszczenie ekranu
        print!("{}[2J", 27 as char);
        
        for _ in 0..box1.p as usize {
            print!(".");
            used += 1;
        }
        for _ in 0..box1.s as usize {
            print!(")");
            used += 1;
        }
        for _ in 0..(box2.p-(box1.p+box1.s)) as usize {
            print!(".");
            used += 1;
        }
        for _ in 0..box2.s as usize {
            print!("(");
            used += 1;
        }
        for _ in 0..(SIZE+MAX_Y - used) {
            print!(".");
        }
        println!("Kolizje: {}", cnt);
        //thread::sleep(frame);
    }
}

//BACKUP
/*
    println!();
    for _ in 0..box1.p as usize {
        print!(".");
    }
    for _ in 0..box1.s as usize {
        print!(")");
    }
    for _ in 0..(box2.p-(box1.p+box1.s)) as usize {
        print!(".");
    }
    for _ in 0..box2.s as usize {
        print!("(");
    }
    for _ in 0..(SIZE as f32 - (box2.p+box2.s)) as usize {
        print!(".");
    }
*/

//BACKUP      

//Program potrafi całość obliczyć w kilka milisekund przy PI_D=7 oraz TIMESTEP=1000000
//Ale drukowanie informacji do terminalu spowalnia program kilkuset krotnie, dlatego
//do drukowania informacji przeznaczam zupełnie inny proces.
 
/*
//kopia aby usunąć potrzebę dodatkowych bibliotek
//takich jak Mutex oraz Lock. Nie jest to najwydajniejsze ale działa
let box1_cop = box1.clone();
let box2_cop = box2.clone();
let cnt_cop = cnt.clone();

//używamy nowego wątka aby nie spowalniać całej reszty
thread::spawn(move|| {
    let mut screen = [['.'; MAX_Y]; SIZE];
    //dziwne, że trzeba resetować, ale to chyba przez automatyczną
    //optymalizację, zapełnianie ekranu kropkami
    for y in 0..MAX_Y {
        for x in 0..SIZE {
            screen[x][y] = '.';
        }
    }
    // bufor rysowania ekranu
    for x in box1_cop.p as usize..box1_cop.s as usize {
        for y in 0..box1_cop.s as usize { 
            screen[x][y] = ')';
        }
    }
    
    //czyszczenie ekranu
    print!("{}[2J", 27 as char);
    
    //rysowanie ekranu
    for y in 0..max_y as usize {
        for x in 0..SIZE {
            print!("{}", screen[x][y]);
        }
        println!();
    }
    println!();
    println!("Kolizje: {}", cnt_cop);
    thread::sleep(frame);
});
*/

