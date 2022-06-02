use yew::prelude::*;
use web_sys::HtmlInputElement;

fn main() {
    println!("Hello, world!");
    yew::start_app::<CounterComponent>();
    println!("{}",15-(15&8));
    //let temp = SudokuBoard::from(vec![1,0,0,0,0,0,0,4,0,0,2,0,0,3,0,0]);
    //let temp = SudokuBoard::from(vec![0,0,0,0,0,0,0,4,0,0,0,0,0,0,0,0]);
    let temp = SudokuBoard::from(vec![1,0,7,0,3,2,0,0,6,
        3,0,6,7,0,0,0,0,0,
        0,8,0,1,9,0,0,5,0,
        0,3,0,2,5,0,0,4,0,
        0,0,9,0,0,0,0,0,0,
        6,0,0,9,0,4,0,0,1,
        0,6,0,4,0,0,0,8,5,
        0,0,0,0,0,0,3,0,0,
        4,1,5,0,7,0,0,0,0]);
    println!("{:?}",temp.unwrap_or(SudokuBoard::sized(1)).fix_nums());//.solve
}

#[derive(Debug, Clone)]
struct SudokuBoard{
    suppos: Vec<u32>,
    celldim: u32,
    dim: u32,
}
impl SudokuBoard{
    fn sized(size: usize) -> SudokuBoard{
        //4294967295
        SudokuBoard{suppos: vec![2_u32.pow((size as f32).sqrt().round() as u32)-1; size],celldim: (size as f32).powf(0.25).round() as u32,dim: (size as f32).sqrt().round() as u32}
    }
    fn from(vals: Vec<u32>) -> Option<SudokuBoard>{
        let mut res: SudokuBoard = SudokuBoard::sized(vals.len());
        for (i,v) in vals.iter().enumerate(){
            if *v!=0 {
                //println!("{} {} {}",i,v,res.dim);
                //println!("{:?}",res.fix_nums());
                if !res.set(i,*v){
                    //println!("{:?}",res.suppos);
                    return Option::None
                }
            }
        }
        Option::Some(res)
    }
    fn set(&mut self, idx: usize, val: u32) -> bool{
        let v: u32 = 2_u32.pow(val-1);
        //println!("{} v {}",v,15&8);
        self.suppos[idx]=v;
        let pos: (usize,usize) = self.get_pos(idx);
        let d: usize = self.celldim as usize;
        let mut cellpos: usize=self.get_idx(((pos.0/d)*d,(pos.1/d) * d));
        let startpos: usize = cellpos;
        for i in 0..self.dim as usize{ //fixes rows and cols
            if i!=pos.0{
                let r: usize = self.get_idx((i,pos.1));
                //println!("setting r {} to {}-{}",r,self.suppos[r],self.suppos[r]&v);
                let new: u32 = self.suppos[r]-(self.suppos[r]&v);
                if new != self.suppos[r]{
                    self.suppos[r]=new;
                    if self.suppos[r]==0 {return false};
                    if self.suppos[r].count_ones()==1 && !self.set(r,32-self.suppos[r].leading_zeros()) {return false};
                }
            }
            if i!=pos.1{
                let c: usize = self.get_idx((pos.0,i)); 
                //println!("setting c {}",c);
                let new: u32 = self.suppos[c]-(self.suppos[c]&v);
                if new != self.suppos[c]{
                    self.suppos[c] = new;
                    if self.suppos[c]==0 {return false};
                    if self.suppos[c].count_ones()==1 && !self.set(c,32-self.suppos[c].leading_zeros()) {return false};
                }
            }//check
            if cellpos != idx{
                let new: u32 = self.suppos[cellpos]-(self.suppos[cellpos]&v);
                //println!("setting pos {}",cellpos);
                if new!=self.suppos[cellpos] {
                    self.suppos[cellpos]=new;
                    if self.suppos[cellpos]==0 {return false};
                    if self.suppos[cellpos].count_ones()==1 && !self.set(cellpos,32-self.suppos[cellpos].leading_zeros()) {return false};
                }
            }
            if cellpos%(self.dim as usize)>=startpos%(self.dim as usize)+d-1{
                cellpos+=self.dim as usize - d + 1;
            }else{cellpos+=1}
        }
        true
    }
    fn solve(&self) -> Option<Vec<u32>>{
        let mut minidx: usize = 0;
        let mut solved: bool = true;
        let mut res: Vec<u32> = Vec::new();
        //if self.suppos[0].count_ones()>1 {solved=false};
        for (i,v) in self.suppos.iter().enumerate(){
            let t: u32 =v.count_ones();
            if t>1{if self.suppos[minidx].count_ones()==1 || t<self.suppos[minidx].count_ones(){minidx=i}; solved = false; res.push(0)}
            else {res.push(32-v.leading_zeros())}
        }
        if solved{return Option::Some(res)};
        let mut copied: u32 = self.suppos[minidx].clone();
        //println!("{} {} {}",minidx, self.suppos[minidx],self.suppos[minidx].count_ones());
        for _i in 0..self.suppos[minidx].count_ones(){
            let mut t: SudokuBoard = self.clone();
            let val: u32 = 32-copied.leading_zeros();
            //println!("idx {} copied {} val {}",minidx, copied,val);
            if t.set(minidx,val){
                //println!("worked!");
                match t.solve(){
                    Some(v) => return Option::Some(v),
                    None => copied-=2_u32.pow(val-1),
                }
            }else {copied-=2_u32.pow(val-1)}
        }
        Option::None
    }
    fn get_idx(&self, pos: (usize, usize)) -> usize{
        pos.0*(self.dim as usize)+pos.1
    }
    fn get_pos(&self, idx: usize) -> (usize,usize){
        let d: usize = self.dim as usize;
        let col: usize = idx%d;
        ((idx-col)/(d),col)
    }
    fn fix_nums(&self) -> Option<Vec<u32>>{
        let mut res: Vec<u32> = Vec::new();
        for v in self.suppos.iter(){
            let t: u32 =v.count_ones();
            if t>1{res.push(0)}
            else {res.push(32-v.leading_zeros())}
        }
        return Option::Some(res)
    }
}

enum Msg{
    UpdateDimentions(u32),
    UpdateContent(u32, u32),
    Solve,
    None,
}

struct CounterComponent{
    cell_dimentions: u32,
    inputs: Vec<u32>,
    solution: Vec<u32>
}

impl Component for CounterComponent{
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self{
        Self{cell_dimentions: 0, inputs: Vec::new(), solution: Vec::new()}
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg{
            Msg::UpdateDimentions(val) =>{
                self.cell_dimentions=val;
                self.inputs=vec![0;(val*val*val*val) as usize];
                if self.solution.len()!=0 {self.solution = Vec::new()}
                true
            } 
            Msg::UpdateContent(idx,val) =>{
                self.inputs[idx as usize] = val;
                false
            }
            Msg::Solve =>{
                //self.solution=self.inputs.clone();
                let temp: Option<SudokuBoard> = SudokuBoard::from(self.inputs.clone());
                
                self.solution = match temp{
                    Some(v) => {match v.solve(){Some(v) => v, None => vec![0,0,0]}},
                    None => {return false}
                };
                /*self.solution = match temp{
                    Some(v) => {match v.fix_nums(){Some(v) => v, None => vec![0,0,0]}},
                    None => {return false}
                };*/
                true
            }
            Msg::None =>{
                //PreviousInput::create(yew::);
                false
            }
        }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html{
        let link = _ctx.link();
        let cols: u32 = &self.cell_dimentions*&self.cell_dimentions;
        let rows: u32 = cols;
        html! {
            <>
                <div class = "i">
                    <h1> {"sudoku solver"} <br/> </h1>
                    <p>{"sudoku solver"}
                    <br/> <br/> </p>
                </div>
                <div class = "inputs">
                    <br/>
                    <label for="celen">{"cell side length: "}</label>
                    <input type="number" id="celen" name="celen" min="0" max="5" oninput={link.callback(|event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::UpdateDimentions(input.value().parse::<u32>().unwrap_or(0))})}/>
                    //<p> <br/> {self.text.clone()} </p>
                </div>
                <div class = "sudoku-board">
                    if self.solution.len()!=0{<div class="grid">{
                        self.solution.iter().enumerate().map(|(i,v)| {
                            html!{
                                <>
                                if (i as u32)%cols==0 {<br/>}
                                if (i as u32)%(rows*self.cell_dimentions)==0 {<br/>}
                                if (i as u32)%self.cell_dimentions==0 {<span class="space">{"  "}</span>}
                                <span class="griditem">
                                //if (i as u32)%rows==0 {<br/>}
                                {v}
                                </span>
                                </>
                                }
                            }).collect::<Html>()
                        }</div>
                    }else if self.cell_dimentions>0{
                        {
                            (0..cols*rows).map(|v: u32| {
                                html!{
                                    <>
                                    if v%cols==0 {<br/>}
                                    if v%(rows*self.cell_dimentions)==0{<br/>}
                                    if v%self.cell_dimentions==0 {<label for={v.to_string()}>{"   "}</label>}
                                    <input type="number" id={v.to_string()} name={v.to_string()} min="0" max="9" oninput={link.callback(move |event: InputEvent| {let input: HtmlInputElement = event.target_unchecked_into(); Msg::UpdateContent(v,input.value().parse::<u32>().unwrap_or(0))})}/>
                                    </>}
                                }).collect::<Html>()
                        }
                    }
                </div>
                <button onclick={link.callback(|_| Msg::Solve)}> {"solve"}</button>
            </>
        }
    }
}