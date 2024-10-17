use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Size},
    Frame,
};

use rand::Rng;
use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        block::{Position, Title},
        canvas::{Canvas, Points},
        Block, BorderType, Borders, Clear, Paragraph,
    },
};
use tui_scrollview::ScrollView;

use crate::{
    app::{App, InputMode},
    simulations::{self, ant::AntSim},
};

use super::{centered_rect_length, render_help};

pub fn ant_screen(frame: &mut Frame, app: &mut App) {
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

    // Initialize the ant simulation if it's not already
    let width = f64::from(frame.area().width - 2);
    let height = f64::from((frame.area().height - 2) * 2);

    if let None = app.ant_sim {
        app.start_ant_default();

        let ant_sim = app.ant_sim.as_mut().unwrap();

        // Initialize the grid with the same size as the canvas
        ant_sim.grid.cells = vec![vec![ant_sim.states[0]; width as usize]; height as usize];

        // Change default ruleset
        ant_sim.rules = AntSim::parse_ant_ruleset("RRLLLRLLLLLLLLL");

        // Set ant position randomly biased towards the center
        let mut rng = rand::thread_rng();

        for ant in &mut ant_sim.ants {
            ant.x = rng.gen_range((width * 0.4) as usize..(width - width * 0.4) as usize) as usize;
            ant.y =
                rng.gen_range((height * 0.4) as usize..(height - height * 0.4) as usize) as usize;
        }

        // Set ant direction randomly
        for ant in &mut ant_sim.ants {
            let direction = rng.gen_range(0..4);
            ant.direction = match direction {
                0 => simulations::Direction::Left,
                1 => simulations::Direction::Right,
                2 => simulations::Direction::Up,
                3 => simulations::Direction::Down,
                _ => simulations::Direction::Right,
            };
        }
    } else if app.ant_sim.as_ref().unwrap().generation == 0 {
        // If the ant simulation is already set, the grid still needs to be initialized with the
        // screen size
        let ant_sim = app.ant_sim.as_mut().unwrap();

        // Initialize the grid with the same size as the canvas
        ant_sim.grid.cells = vec![vec![ant_sim.states[0]; width as usize]; height as usize];

        // Reposition the ant inside the bounds if it is outside
        for ant in ant_sim.ants.iter_mut() {
            if ant.x > width as usize {
                ant.x = width as usize / 2;
            }

            if ant.y > height as usize {
                ant.y = height as usize / 2;
            }
        }
    }

    // From here `app.ant_sim` is `Some`
    let ant_sim = app.ant_sim.as_ref().unwrap();

    /////////////////////////////
    // Border content
    /////////////////////////////

    let top_title = Title::from(Line::from(vec![" Langton's Ant ".yellow()]))
        .position(Position::Top)
        .alignment(Alignment::Center);

    let bottom_left_title = Title::from(Line::from(vec![
        " Iteration: ".into(),
        ant_sim.generation.to_string().yellow(),
        " ".into(),
    ]))
    .position(Position::Bottom);

    let key_help = Title::from(Line::from(vec![" '?' ".yellow(), "Help ".into()]))
        .position(Position::Bottom)
        .alignment(Alignment::Center);

    let bottom_right_title = Title::from(Line::from(vec![
        " Speed: ".into(),
        if app.speed.as_millis() == 0 {
            format!("{}x ", app.speed_multiplier).yellow()
        } else {
            format!("{}ms ", app.speed.as_millis()).yellow()
        },
    ]))
    .position(Position::Bottom)
    .alignment(Alignment::Right);

    /* let top_left_debug = Title::from(Line::from(vec![
        "(".into(),
        ant_sim.ants[0].x.to_string().yellow(),
        "/".into(),
        ant_sim.grid.width().to_string().red(),
        ",".into(),
        ant_sim.ants[0].y.to_string().yellow(),
        "/".into(),
        ant_sim.grid.height().to_string().red(),
        ")".into(),
        " ".into(),
        ant_sim.ants[0].direction.to_string().yellow(),
        " ".into(),
        Span::styled(
            ant_sim.states[ant_sim.generation % ant_sim.states.len()].to_string(),
            Style::default().fg(ant_sim.states[ant_sim.generation % ant_sim.states.len()]),
        ),
        " ".into(),
        "[".into(),
        width.to_string().red(),
        ",".into(),
        height.to_string().red(),
        "]".into(),
        " ".into(),
    ])); */

    /////////////////////////////
    // Simulation canvas
    /////////////////////////////

    let ant_canvas = Canvas::default()
        .block(
            Block::default()
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                // .title(top_left_debug)
                .title(top_title)
                .title(bottom_left_title)
                .title(bottom_right_title)
                .title(key_help)
                .title_style(Style::default().bold()),
        )
        .marker(app.marker)
        .paint(|ctx| {
            // Draw grid
            for (y, row) in ant_sim.grid.cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    match *cell {
                        // Skip drawing black cells
                        Color::Black => {}
                        _ => {
                            ctx.draw(&Points {
                                coords: &[(x as f64, y as f64)],
                                color: *cell,
                            });
                        }
                    }
                }
            }

            // Draw ant
            for ant in ant_sim.ants.iter() {
                ctx.draw(&Points {
                    coords: &[(ant.x as f64, ant.y as f64)],
                    color: ant.color,
                });
            }
        })
        .x_bounds([0., f64::from((frame.area().width - 2) - 1)])
        .y_bounds([0., f64::from(((frame.area().height - 2) * 2) - 1)]);

    frame.render_widget(ant_canvas, frame.area());

    /////////////////////////////
    // Help screen
    /////////////////////////////

    let help_entries: Vec<(Line, Line)> = vec![
        (Line::from("Q / Esc".yellow()), Line::from("Quit")),
        (Line::from("?".yellow()), Line::from("Help")),
        (Line::from("Space".yellow()), Line::from("Start/Pause")),
        (Line::from("K / ↑".yellow()), Line::from("Speed Up")),
        (Line::from("J / ↓".yellow()), Line::from("Speed Down")),
        (Line::from("L / →".yellow()), Line::from("Next Generation")),
    ];

    if app.help_screen {
        render_help(frame, help_entries);
    }
}

pub fn edit(frame: &mut Frame, app: &mut App) {
    let ant_sim = app.ant_sim.as_mut().unwrap();

    let edit_area_width = 27;
    let edit_area_height = 10;

    let edit_area = centered_rect_length(
        edit_area_width,
        edit_area_height,
        frame.area(),
    );

    // Area with offsets for the border
    let scroll_area = Rect::new(edit_area.x + 1, edit_area.y + 1, edit_area_width - 1, edit_area_height - 2);

    let mut scroll_view = ScrollView::new(Size::new(edit_area_width - 2, 50));

    let edit_block = Block::default()
        .title(" Edit ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(Clear, edit_area);
    frame.render_widget(edit_block, edit_area);

    let ruleset_layout_v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(edit_area);

    let ruleset_layout_h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(ruleset_layout_v[1]);

    let input_scroll = ant_sim
        .rules_input
        .visual_scroll(edit_area_width.saturating_sub(5) as usize);

    let input = Paragraph::new(ant_sim.rules_input.value())
        .scroll((0, input_scroll as u16))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(match ant_sim.rules_input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => Style::default().yellow().bold(),
                })
                .title(" Input "),
        );

    scroll_view.render_widget(input, Rect::new(0, 0, edit_area_width, 3));

    match ant_sim.rules_input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // TODO: Fix this to be dynamic inside scroll view
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            frame.set_cursor_position((
                // Put cursor past the end of the input text
                ruleset_layout_h[1].x
                    + ((ant_sim.rules_input.visual_cursor()).saturating_sub(input_scroll)) as u16
                    + 1,
                // Move one line down, from the border to the input line
                ruleset_layout_h[1].y + 1,
            ))
        }
    }

    frame.render_stateful_widget(scroll_view, scroll_area, &mut ant_sim.scroll_state);
}
