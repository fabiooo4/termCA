use std::cell::Cell;

use ratatui::{
    layout::{Margin, Rect},
    text::{Line, Text},
    widgets::{Row, Scrollbar, ScrollbarOrientation, Table},
    Frame,
};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tui_big_text::{BigText, PixelSize};

use crate::app::{App, Screen};

use super::{ant_ui, render_help};

pub fn main_screen(frame: &mut Frame, app: &mut App) {
    if frame
        .area()
        .width
        .checked_mul(frame.area().height)
        .is_none()
    {
        let error_paragraph = Paragraph::new(
            "EEEEEEEEEEEEEEEEEEEEEE                                                                                                   tttt         hhhhhhh                                                         hhhhhhh                                                                                              tttt                                                                          iiii                                              iiii                                 tttt                                                                                                            lllllll lllllll 
E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E                                                                                                ttt⣿⣿⣿t         h⣿⣿⣿⣿⣿h                                                         h⣿⣿⣿⣿⣿h                                                                                           ttt⣿⣿⣿t                                                                         i⣿⣿⣿⣿i                                            i⣿⣿⣿⣿i                             ttt⣿⣿⣿t                                                                                                            l⣿⣿⣿⣿⣿l l⣿⣿⣿⣿⣿l 
E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E                                                                                                t⣿⣿⣿⣿⣿t         h⣿⣿⣿⣿⣿h                                                         h⣿⣿⣿⣿⣿h                                                                                           t⣿⣿⣿⣿⣿t                                                                          iiii                                              iiii                              t⣿⣿⣿⣿⣿t                                                                                                            l⣿⣿⣿⣿⣿l l⣿⣿⣿⣿⣿l 
EE⣿⣿⣿⣿⣿⣿EEEEEEEEE⣿⣿⣿⣿E                                                                                                t⣿⣿⣿⣿⣿t         h⣿⣿⣿⣿⣿h                                                         h⣿⣿⣿⣿⣿h                                                                                           t⣿⣿⣿⣿⣿t                                                                                                                                                              t⣿⣿⣿⣿⣿t                                                                                                            l⣿⣿⣿⣿⣿l l⣿⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿E       EEEEEErrrrr   rrrrrrrrr   rrrrr   rrrrrrrrr      ooooooooooo   rrrrr   rrrrrrrrr                ttttttt⣿⣿⣿⣿⣿ttttttt    h⣿⣿⣿⣿h hhhhh           eeeeeeeeeeee             cccccccccccccccch⣿⣿⣿⣿h hhhhh         aaaaaaaaaaaaa  rrrrr   rrrrrrrrr   aaaaaaaaaaaaa      ccccccccccccccccttttttt⣿⣿⣿⣿⣿ttttttt        eeeeeeeeeeee    rrrrr   rrrrrrrrr            ssssssssss   iiiiiii zzzzzzzzzzzzzzzzz    eeeeeeeeeeee         iiiiiii     ssssssssss        ttttttt⣿⣿⣿⣿⣿ttttttt       ooooooooooo      ooooooooooo            ssssssssss      mmmmmmm    mmmmmmm     aaaaaaaaaaaaa    l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿E             r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r  r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r   oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r   ⣿⣿⣿⣿⣿⣿      t⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t    h⣿⣿⣿⣿hh⣿⣿⣿⣿⣿hhh      ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ee         cc⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ch⣿⣿⣿⣿hh⣿⣿⣿⣿⣿hhh      a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r  a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a   cc⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ct⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t      ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ee  r⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿⣿⣿⣿r         ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s  i⣿⣿⣿⣿⣿i z⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿z  ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ee       i⣿⣿⣿⣿⣿i   ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s       t⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t     oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo  oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo        ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s   mm⣿⣿⣿⣿⣿⣿⣿m  m⣿⣿⣿⣿⣿⣿⣿mm   a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a   l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿⣿EEEEEEEEEE   r⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r r⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r o⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿or⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r  ⣿⣿⣿⣿⣿⣿      t⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t    h⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿hh   e⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿ee      c⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿hh    aaaaaaaaa⣿⣿⣿⣿⣿ar⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r aaaaaaaaa⣿⣿⣿⣿⣿a c⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ct⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t     e⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿eer⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿r      ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s  i⣿⣿⣿⣿i z⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿z  e⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿ee      i⣿⣿⣿⣿i ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s      t⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t    o⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿o     ss⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s m⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿mm⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿m  aaaaaaaaa⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E   rr⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿rrr⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿ro⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿orr⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿r ⣿⣿⣿⣿⣿⣿      tttttt⣿⣿⣿⣿⣿⣿⣿tttttt    h⣿⣿⣿⣿⣿⣿⣿hhh⣿⣿⣿⣿⣿⣿h e⣿⣿⣿⣿⣿⣿e     e⣿⣿⣿⣿⣿e     c⣿⣿⣿⣿⣿⣿⣿cccccc⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿⣿⣿hhh⣿⣿⣿⣿⣿⣿h            a⣿⣿⣿⣿arr⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿r         a⣿⣿⣿⣿ac⣿⣿⣿⣿⣿⣿⣿cccccc⣿⣿⣿⣿⣿ctttttt⣿⣿⣿⣿⣿⣿⣿tttttt    e⣿⣿⣿⣿⣿⣿e     e⣿⣿⣿⣿⣿err⣿⣿⣿⣿⣿⣿rrrrr⣿⣿⣿⣿⣿⣿r     s⣿⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿s i⣿⣿⣿⣿i zzzzzzzz⣿⣿⣿⣿⣿⣿z  e⣿⣿⣿⣿⣿⣿e     e⣿⣿⣿⣿⣿e      i⣿⣿⣿⣿i s⣿⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿s     tttttt⣿⣿⣿⣿⣿⣿⣿tttttt    o⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿oo⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿o     s⣿⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿sm⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿m           a⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E    r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿r r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿ro⣿⣿⣿⣿o     o⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿r                   t⣿⣿⣿⣿⣿t          h⣿⣿⣿⣿⣿⣿h   h⣿⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿⣿e     c⣿⣿⣿⣿⣿⣿c     ccccccch⣿⣿⣿⣿⣿⣿h   h⣿⣿⣿⣿⣿⣿h    aaaaaaa⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿r  aaaaaaa⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿⣿c     ccccccc      t⣿⣿⣿⣿⣿t          e⣿⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿⣿e r⣿⣿⣿⣿⣿r     r⣿⣿⣿⣿⣿r      s⣿⣿⣿⣿⣿s  ssssss  i⣿⣿⣿⣿i       z⣿⣿⣿⣿⣿⣿z   e⣿⣿⣿⣿⣿⣿⣿eeeee⣿⣿⣿⣿⣿⣿e      i⣿⣿⣿⣿i  s⣿⣿⣿⣿⣿s  ssssss            t⣿⣿⣿⣿⣿t          o⣿⣿⣿⣿o     o⣿⣿⣿⣿oo⣿⣿⣿⣿o     o⣿⣿⣿⣿o      s⣿⣿⣿⣿⣿s  ssssss m⣿⣿⣿⣿⣿mmm⣿⣿⣿⣿⣿⣿mmm⣿⣿⣿⣿⣿m    aaaaaaa⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿⣿EEEEEEEEEE    r⣿⣿⣿⣿⣿r     rrrrrrr r⣿⣿⣿⣿⣿r     rrrrrrro⣿⣿⣿⣿o     o⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r     rrrrrrr                   t⣿⣿⣿⣿⣿t          h⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e      c⣿⣿⣿⣿⣿c             h⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h  aa⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r     rrrrrrraa⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿c                   t⣿⣿⣿⣿⣿t          e⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e  r⣿⣿⣿⣿⣿r     rrrrrrr        s⣿⣿⣿⣿⣿⣿s       i⣿⣿⣿⣿i      z⣿⣿⣿⣿⣿⣿z    e⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e       i⣿⣿⣿⣿i    s⣿⣿⣿⣿⣿⣿s                 t⣿⣿⣿⣿⣿t          o⣿⣿⣿⣿o     o⣿⣿⣿⣿oo⣿⣿⣿⣿o     o⣿⣿⣿⣿o        s⣿⣿⣿⣿⣿⣿s      m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿m  aa⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿E              r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r            o⣿⣿⣿⣿o     o⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r                               t⣿⣿⣿⣿⣿t          h⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿eeeeeeeeeee       c⣿⣿⣿⣿⣿c             h⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h a⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r           a⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿c                   t⣿⣿⣿⣿⣿t          e⣿⣿⣿⣿⣿⣿eeeeeeeeeee   r⣿⣿⣿⣿⣿r                       s⣿⣿⣿⣿⣿⣿s    i⣿⣿⣿⣿i     z⣿⣿⣿⣿⣿⣿z     e⣿⣿⣿⣿⣿⣿eeeeeeeeeee        i⣿⣿⣿⣿i       s⣿⣿⣿⣿⣿⣿s              t⣿⣿⣿⣿⣿t          o⣿⣿⣿⣿o     o⣿⣿⣿⣿oo⣿⣿⣿⣿o     o⣿⣿⣿⣿o           s⣿⣿⣿⣿⣿⣿s   m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿m a⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
  E⣿⣿⣿⣿⣿E       EEEEEE r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r            o⣿⣿⣿⣿o     o⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r             ⣿⣿⣿⣿⣿⣿            t⣿⣿⣿⣿⣿t    tttttth⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿⣿e                c⣿⣿⣿⣿⣿⣿c     ccccccch⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿ha⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r          a⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿⣿c     ccccccc      t⣿⣿⣿⣿⣿t    tttttte⣿⣿⣿⣿⣿⣿⣿e            r⣿⣿⣿⣿⣿r                 ssssss   s⣿⣿⣿⣿⣿s  i⣿⣿⣿⣿i    z⣿⣿⣿⣿⣿⣿z      e⣿⣿⣿⣿⣿⣿⣿e                 i⣿⣿⣿⣿i ssssss   s⣿⣿⣿⣿⣿s            t⣿⣿⣿⣿⣿t    tttttto⣿⣿⣿⣿o     o⣿⣿⣿⣿oo⣿⣿⣿⣿o     o⣿⣿⣿⣿o     ssssss   s⣿⣿⣿⣿⣿s m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿ma⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿a  l⣿⣿⣿⣿l  l⣿⣿⣿⣿l 
EE⣿⣿⣿⣿⣿⣿EEEEEEEE⣿⣿⣿⣿⣿E r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r            o⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r             ⣿⣿⣿⣿⣿⣿            t⣿⣿⣿⣿⣿⣿tttt⣿⣿⣿⣿⣿th⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿he⣿⣿⣿⣿⣿⣿⣿⣿e               c⣿⣿⣿⣿⣿⣿⣿cccccc⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿ha⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r          a⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿ac⣿⣿⣿⣿⣿⣿⣿cccccc⣿⣿⣿⣿⣿c      t⣿⣿⣿⣿⣿⣿tttt⣿⣿⣿⣿⣿te⣿⣿⣿⣿⣿⣿⣿⣿e           r⣿⣿⣿⣿⣿r                 s⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿⣿si⣿⣿⣿⣿⣿⣿i  z⣿⣿⣿⣿⣿⣿zzzzzzzze⣿⣿⣿⣿⣿⣿⣿⣿e               i⣿⣿⣿⣿⣿⣿is⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿⣿s           t⣿⣿⣿⣿⣿⣿tttt⣿⣿⣿⣿⣿to⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿oo⣿⣿⣿⣿⣿ooooo⣿⣿⣿⣿⣿o     s⣿⣿⣿⣿⣿ssss⣿⣿⣿⣿⣿⣿sm⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿ma⣿⣿⣿⣿a    a⣿⣿⣿⣿⣿a l⣿⣿⣿⣿⣿⣿ll⣿⣿⣿⣿⣿⣿l
E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r            o⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿o r⣿⣿⣿⣿⣿r             ⣿⣿⣿⣿⣿⣿            tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿th⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h e⣿⣿⣿⣿⣿⣿⣿⣿eeeeeeee        c⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿ha⣿⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a r⣿⣿⣿⣿⣿r          a⣿⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a c⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿c      tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿t e⣿⣿⣿⣿⣿⣿⣿⣿eeeeeeee   r⣿⣿⣿⣿⣿r                 s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s i⣿⣿⣿⣿⣿⣿i z⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿z e⣿⣿⣿⣿⣿⣿⣿⣿eeeeeeee       i⣿⣿⣿⣿⣿⣿is⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s            tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿to⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿o     s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿s m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿ma⣿⣿⣿⣿⣿aaaa⣿⣿⣿⣿⣿⣿a l⣿⣿⣿⣿⣿⣿ll⣿⣿⣿⣿⣿⣿l
E⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿E r⣿⣿⣿⣿⣿r             r⣿⣿⣿⣿⣿r             oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo  r⣿⣿⣿⣿⣿r                                 tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿tth⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h  ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e         cc⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ch⣿⣿⣿⣿⣿h     h⣿⣿⣿⣿⣿h a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿aa⣿⣿⣿ar⣿⣿⣿⣿⣿r           a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿aa⣿⣿⣿a cc⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿c        tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿tt  ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e   r⣿⣿⣿⣿⣿r                  s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ss  i⣿⣿⣿⣿⣿⣿iz⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿z  ee⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿e       i⣿⣿⣿⣿⣿⣿i s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ss               tt⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿tt oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo  oo⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿oo       s⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿ss  m⣿⣿⣿⣿m   m⣿⣿⣿⣿m   m⣿⣿⣿⣿m a⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿aa⣿⣿⣿al⣿⣿⣿⣿⣿⣿ll⣿⣿⣿⣿⣿⣿l
EEEEEEEEEEEEEEEEEEEEEE rrrrrrr             rrrrrrr               ooooooooooo    rrrrrrr                                   ttttttttttt  hhhhhhh     hhhhhhh    eeeeeeeeeeeeee           cccccccccccccccchhhhhhh     hhhhhhh  aaaaaaaaaa  aaaarrrrrrr            aaaaaaaaaa  aaaa   cccccccccccccccc          ttttttttttt      eeeeeeeeeeeeee   rrrrrrr                   sssssssssss    iiiiiiiizzzzzzzzzzzzzzzzz    eeeeeeeeeeeeee       iiiiiiii  sssssssssss                   ttttttttttt     ooooooooooo      ooooooooooo          sssssssssss    mmmmmm   mmmmmm   mmmmmm  aaaaaaaaaa  aaaallllllllllllllll"
        ).alignment(Alignment::Center);
        frame.render_widget(
            error_paragraph,
            Rect {
                x: 0,
                y: 0,
                width: 592,
                height: 16,
            },
        );
        return;
    }

    /////////////////////////////
    // Border
    /////////////////////////////

    let key_help = Line::from(vec![" '?' ".yellow(), "Help ".into()]);
    let border = Block::default()
        .title_bottom(key_help.centered())
        .borders(Borders::ALL)
        .border_type(BorderType::Double);

    frame.render_widget(border, frame.area());

    /////////////////////////////
    // Layout
    /////////////////////////////
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([
            Constraint::Min(0),    // Spacing
            Constraint::Length(4), // Title
            Constraint::Length(1), // Subtitle
            Constraint::Max(3),    // Spacing
            Constraint::Min(8),    // List block
            Constraint::Min(0),    // Spacing
        ])
        .split(frame.area());

    let longest_sim = app
        .list_items
        .iter()
        .map(|i| i.label.len())
        .max()
        .unwrap_or(1);
    let longest_edit = 4;

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(longest_sim as u16 + longest_edit as u16 + 6),
            Constraint::Min(1),
        ])
        .split(vertical_layout[4]);

    let list_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(2)
        .vertical_margin(1)
        .constraints([Constraint::Min(1)])
        .split(horizontal_layout[1]);

    /////////////////////////////
    // Titles
    /////////////////////////////
    let title = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .style(Style::new().yellow().bold())
        .lines(vec!["TermCA".into()])
        .alignment(Alignment::Center)
        .build();

    let subtitle = Paragraph::new("Cellular Automata".to_string())
        .alignment(Alignment::Center)
        .white()
        .bold();

    frame.render_widget(title, vertical_layout[1]);
    frame.render_widget(subtitle, vertical_layout[2]);

    /////////////////////////////
    // List
    /////////////////////////////
    let list_border = Block::bordered()
        .border_type(BorderType::Rounded)
        .style(Style::new().yellow());

    frame.render_widget(list_border, horizontal_layout[1]);

    let list_items: Vec<Row> = app
        .list_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            Row::new(vec![
                Text::from(format!("{}\n", item.label.clone())),
                if i != app.list_items.len() - 1 {
                    Text::from("Edit\n".to_string())
                } else {
                    Text::from("\n".to_string())
                },
            ])
            .style(Style::default().white().dim())
            .height(2)
        })
        .collect();

    // Highlight added to the selected row, but on the column that is not selected
    let selected_row_style = Style::default().white().bold().not_dim();
    let selected_cell_style = Style::default().yellow().bold().not_dim();

    let list = Table::new(
        list_items,
        [
            Constraint::Length(longest_sim as u16 + 1),
            Constraint::Min(longest_edit as u16 + 1),
        ],
    )
    .row_highlight_style(selected_row_style)
    .cell_highlight_style(selected_cell_style);

    frame.render_stateful_widget(list, list_layout[0], &mut app.list_state);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    let offsets_number: usize = ((app.list_items.len() * 2) as f32
        / (horizontal_layout[1].height - 2) as f32)
        .ceil() as usize;

    app.scroll_state = app.scroll_state.content_length(offsets_number);

    if horizontal_layout[1].height - 2 < app.list_items.len() as u16 * 2 {
        frame.render_stateful_widget(
            scrollbar,
            horizontal_layout[1].inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut app.scroll_state,
        );
    }

    /////////////////////////////
    // Help screen
    /////////////////////////////

    let help_entries: Vec<(Line, Line)> = vec![
        (Line::from("Q / Esc".yellow()), Line::from("Quit")),
        (Line::from("?".yellow()), Line::from("Help")),
        (Line::from("Enter".yellow()), Line::from("Select")),
        (Line::from("K / ↑".yellow()), Line::from("Scroll Up")),
        (Line::from("J / ↓".yellow()), Line::from("Scroll Down")),
        (Line::from("L / →".yellow()), Line::from("Scroll Right")),
        (Line::from("H / ←".yellow()), Line::from("Scroll Left")),
        (Line::from("g".yellow()), Line::from("Scroll to Bottom")),
        (Line::from("G".yellow()), Line::from("Scroll to Top")),
    ];

    if app.help_screen && app.editing.is_none() {
        render_help(frame, help_entries);
    }

    /////////////////////////////
    // Edit screen
    /////////////////////////////
    if let Some(edit_sim) = app.editing {
        match edit_sim {
            Screen::Ant => ant_ui::edit(frame, app),
            Screen::AntEdit(ant_idx) => ant_ui::edit_ant(frame, app, ant_idx),
            _ => {}
        }
    }
}
