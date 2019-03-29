<?php
/**
 * @param int $begin go to work
 * @param int $end go home
 * @param integer $days week work days
 * @return bool take icu.
 */

function icu($begin = 9, $end = 17, $days = 5)
{
    /** @var int $times day work times */
    $times = $end - $begin;
    return 40 / ($times * $days);
}

$health = round((icu(9, 21, 6) - 1) * 100, 2);
echo 'health = ' . $health . '%';
