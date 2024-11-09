use ratatui::{layout::Rect, Frame};

use ratatui::{
    layout::Alignment,
    style::{Style, Stylize},
    text::Line,
    widgets::{
        canvas::{Canvas, Points},
        Block, BorderType, Borders, Paragraph,
    },
};

use crate::
    app::App
;

use super::render_help;

pub fn elementary_screen(frame: &mut Frame, app: &mut App) {
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

    // Initialize the simulation if it's not already
    let width = f64::from(frame.area().width - 2);
    let height = f64::from((frame.area().height - 2) * 2);

    if app.elementary_sim.is_none() {
        app.start_elementary_default();

        let sim = app.elementary_sim.as_mut().unwrap();

        // Initialize the grid with the same size as the canvas
        sim.grid
            .resize(width as usize, height as usize, sim.dead_state);
        sim.current_line.resize(width as usize, false);

        // Set the middle element of the first generation to true
        sim.current_line.insert(width as usize / 2, true);
    } else if app.elementary_sim.as_ref().unwrap().generation == 0 {
        // If the simulation is already set, the grid still needs to be initialized with the
        // screen size
        let sim = app.elementary_sim.as_mut().unwrap();

        // Initialize the grid with the same size as the canvas
        sim.grid
            .resize(width as usize, height as usize, sim.dead_state);
        sim.current_line.resize(width as usize, false);

        // Set the middle element of the first generation to true
        sim.current_line.insert(width as usize / 2, true);
    }

    // From here `app.elementary_sim` is `Some`
    let sim = app.elementary_sim.as_ref().unwrap();

    /////////////////////////////
    // Border content
    /////////////////////////////

    let top_title = Line::from(vec![" Elementary CA ".yellow()]);

    let bottom_left_title = Line::from(vec![
        " Iteration: ".into(),
        sim.generation.to_string().yellow(),
        " ".into(),
    ]);

    let key_help = Line::from(vec![" '?' ".yellow(), "Help ".into()]);

    let bottom_right_title = Line::from(vec![
        " Speed: ".into(),
        if app.speed.as_millis() == 0 {
            format!("{}x ", app.speed_multiplier).yellow()
        } else {
            format!("{}ms ", app.speed.as_millis()).yellow()
        },
    ]);

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
        ratatui::text::Span::styled(
            ant_sim.grid.cells[ant_sim.ants[0].y][ant_sim.ants[0].x].to_string(),
            Style::default().fg(ant_sim.grid.cells[ant_sim.ants[0].y][ant_sim.ants[0].x]),
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

    let canvas = Canvas::default()
        .block(
            Block::default()
                .border_type(BorderType::Double)
                .borders(Borders::ALL)
                // .title(top_left_debug)
                .title_top(top_title.centered())
                .title_bottom(bottom_left_title.left_aligned())
                .title_bottom(bottom_right_title.right_aligned())
                .title_bottom(key_help.centered())
                .title_style(Style::default().bold()),
        )
        .marker(app.marker)
        .paint(|ctx| {
            // Draw grid
            for (y, row) in sim.grid.cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    ctx.draw(&Points {
                        coords: &[(x as f64, y as f64)],
                        color: *cell,
                    });
                }
            }
        })
        .x_bounds([0., f64::from((frame.area().width - 2) - 1)])
        .y_bounds([0., f64::from(((frame.area().height - 2) * 2) - 1)]);

    frame.render_widget(canvas, frame.area());

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